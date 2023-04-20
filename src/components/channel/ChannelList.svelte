<script lang="ts">
    import { currentServer, ws } from "../../stores/current";
    import User from "../User.svelte";
    import ChannelIcon from "./ChannelIcon.svelte";

    const addChannel = async () => {
        if ($currentServer)
            $ws?.send(
                JSON.stringify({
                    action: "CreateChannel",

                    data: {
                        name: "channel",
                        channel_type: "text",
                        server_id: parseInt($currentServer?.id),
                    },
                })
            );
    };
</script>

<div class="channels">
    <div class="title">
        <span />
        <p>Channels</p>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i class="fa-solid fa-plus add" on:click={addChannel} />
    </div>

    <div class="list">
        {#each $currentServer?.channels || [] as channel}
            <ChannelIcon {...channel} />
        {/each}
    </div>

    <User />
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

            margin: 0;
            padding: 4% 0;

            color: white;
            font-family: "Ubuntu";

            display: grid;
            grid-template-columns: repeat(3, calc(100% / 3));
            align-items: center;
            justify-content: center;
            align-content: center;

            span,
            p {
                margin: 0;
                padding: 0;
            }

            .add {
                justify-self: right;
                margin-right: 18%;
                cursor: pointer;

                border: 1px solid rgba(255, 255, 255, 0.4);
                border-radius: 4px;
                padding: 5% 6%;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                text-align: center;

                transition: border-color 0.5s ease, background-color 0.5s ease, color 0.5s ease;

                &:hover {
                    color: black;
                    border-color: transparent;
                    background-color: rgba(255, 255, 255, 0.6);
                }

                &::after {
                    width: 100%;
                    height: 100%;

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    text-align: center;
                }
            }
        }

        .list {
            overflow-y: scroll;
            width: 100%;
            height: 88%;
            overflow-x: hidden;
        }
    }
</style>
