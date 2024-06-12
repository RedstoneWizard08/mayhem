use super::signal;
use anyhow::Result;
use std::sync::Arc;
use tokio::time::Duration;

use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication,
    rtp_transceiver::rtp_codec::RTPCodecType,
    track::track_local::{
        track_local_static_rtp::TrackLocalStaticRTP, TrackLocal, TrackLocalWriter,
    },
    Error,
};

pub async fn start_broadcaster() -> Result<()> {
    let mut sdp_chan_rx = signal::http_sdp_server(4002).await;

    println!("wait for the offer from http_sdp_server\n");

    let line = sdp_chan_rx.recv().await.unwrap();
    let desc_data = signal::decode(line.as_str())?;
    let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;

    println!("Receive offer from http_sdp_server:\n{:?}", offer);

    let mut m = MediaEngine::default();

    m.register_default_codecs()?;

    let mut registry = Registry::new();

    registry = register_default_interceptors(registry, &mut m)?;

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Video, None)
        .await?;

    let (local_track_chan_tx, mut local_track_chan_rx) =
        tokio::sync::mpsc::channel::<Arc<TrackLocalStaticRTP>>(1);

    let local_track_chan_tx = Arc::new(local_track_chan_tx);
    let pc = Arc::downgrade(&peer_connection);

    peer_connection.on_track(Box::new(move |track, _, _| {
        let media_ssrc = track.ssrc();
        let pc2 = pc.clone();

        tokio::spawn(async move {
            let mut result = Result::<usize>::Ok(0);

            while result.is_ok() {
                let timeout = tokio::time::sleep(Duration::from_secs(3));

                tokio::pin!(timeout);

                tokio::select! {
                    _ = timeout.as_mut() =>{
                        if let Some(pc) = pc2.upgrade(){
                            result = pc.write_rtcp(&[Box::new(PictureLossIndication{
                                sender_ssrc: 0,
                                media_ssrc,
                            })]).await.map_err(Into::into);
                        }else{
                            break;
                        }
                    }
                };
            }
        });

        let local_track_chan_tx2 = Arc::clone(&local_track_chan_tx);

        tokio::spawn(async move {
            let local_track = Arc::new(TrackLocalStaticRTP::new(
                track.codec().capability,
                "video".to_owned(),
                "webrtc-rs".to_owned(),
            ));

            let _ = local_track_chan_tx2.send(Arc::clone(&local_track)).await;

            while let Ok((rtp, _)) = track.read_rtp().await {
                if let Err(err) = local_track.write_rtp(&rtp).await {
                    if Error::ErrClosedPipe != err {
                        print!("output track write_rtp got error: {err} and break");
                        break;
                    } else {
                        print!("output track write_rtp got error: {err}");
                    }
                }
            }
        });

        Box::pin(async {})
    }));

    peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {s}");
        Box::pin(async {})
    }));

    peer_connection.set_remote_description(offer).await?;

    let answer = peer_connection.create_answer(None).await?;
    let mut gather_complete = peer_connection.gathering_complete_promise().await;

    peer_connection.set_local_description(answer).await?;

    let _ = gather_complete.recv().await;

    if let Some(local_desc) = peer_connection.local_description().await {
        let json_str = serde_json::to_string(&local_desc)?;
        let b64 = signal::encode(&json_str);

        println!("{b64}");
    } else {
        println!("generate local_description failed!");
    }

    if let Some(local_track) = local_track_chan_rx.recv().await {
        loop {
            println!("\nCurl an base64 SDP to start sendonly peer connection");

            let line = sdp_chan_rx.recv().await.unwrap();
            let desc_data = signal::decode(line.as_str())?;
            let recv_only_offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;

            let mut m = MediaEngine::default();

            m.register_default_codecs()?;

            let mut registry = Registry::new();

            registry = register_default_interceptors(registry, &mut m)?;

            let api = APIBuilder::new()
                .with_media_engine(m)
                .with_interceptor_registry(registry)
                .build();

            let config = RTCConfiguration {
                ice_servers: vec![RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            };

            let peer_connection = Arc::new(api.new_peer_connection(config).await?);

            let rtp_sender = peer_connection
                .add_track(Arc::clone(&local_track) as Arc<dyn TrackLocal + Send + Sync>)
                .await?;

            tokio::spawn(async move {
                let mut rtcp_buf = vec![0u8; 1500];
                while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
                Result::<()>::Ok(())
            });

            peer_connection.on_peer_connection_state_change(Box::new(
                move |s: RTCPeerConnectionState| {
                    println!("Peer Connection State has changed: {s}");
                    Box::pin(async {})
                },
            ));

            peer_connection
                .set_remote_description(recv_only_offer)
                .await?;

            let answer = peer_connection.create_answer(None).await?;
            let mut gather_complete = peer_connection.gathering_complete_promise().await;

            peer_connection.set_local_description(answer).await?;

            let _ = gather_complete.recv().await;

            if let Some(local_desc) = peer_connection.local_description().await {
                let json_str = serde_json::to_string(&local_desc)?;
                let b64 = signal::encode(&json_str);

                println!("{b64}");
            } else {
                println!("generate local_description failed!");
            }
        }
    }

    return Ok(());
}
