<script lang="ts">
    import { currentServer, ws } from "../../stores/current";
    import ChannelIcon from "./ChannelIcon.svelte";

    const addChannel = async () => {
        if ($currentServer)
            $ws?.send(
                JSON.stringify({
                    action: "CreateChannel",
                
                    data: {
                        name: "channel",
                        channel_type: "text",
                        server_id: parseInt($currentServer?.id)
                    },
                })
            );
    };
</script>

<div class="channels">
    <p class="title">
        <span />
        Channels
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i class="fa-solid fa-plus add" on:click={addChannel} />
    </p>

    {#each $currentServer?.channels || [] as channel}
        <ChannelIcon {...channel} />
    {/each}
</div>

<style lang="scss">
    .channels {
        background-color: var(--color-background-light);

        height: 100%;
        width: 20%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        border-left: 1px solid rgba(255, 255, 255, 0.1);

        .title {
            width: 100%;
            height: 4%;

            color: white;
            font-family: "Ubuntu";

            display: grid;
            grid-template-columns: repeat(3, calc(100% / 3));
            align-items: center;
            justify-content: center;
            align-content: center;

            .add {
                justify-self: right;
                margin-right: 18%;
                cursor: pointer;

                border: 1px solid rgba(255, 255, 255, 0.4);
                border-radius: 4px;
                padding: 4% 5%;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                transition: background-color 0.5s ease, color 0.5s ease;

                &:hover {
                    color: black;
                    background-color: rgba(255, 255, 255, 0.6);
                }
            }
        }
    }
</style>
