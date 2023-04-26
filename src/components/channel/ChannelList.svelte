<script lang="ts">
    import { currentServer, ws } from "../../stores/current";
    import Modal from "../Modal.svelte";
    import User from "../User.svelte";
    import ChannelIcon from "./ChannelIcon.svelte";

    let channelName = "";

    $: creatingChannel = false;

    const createChannel = async () => {
        if ($currentServer)
            $ws?.send(
                JSON.stringify({
                    action: "CreateChannel",

                    data: {
                        name: channelName,
                        channel_type: "text",
                        server_id: parseInt($currentServer?.id),
                    },
                })
            );
        
        creatingChannel = false;
        channelName = "";
    };
</script>

<Modal name="Create Channel" open={creatingChannel} width="40%" height="40%">
    <div class="channel-modal">
        <input
            type="text"
            class="channel-name"
            placeholder="Channel name..."
            bind:value={channelName}
        />

        <button type="button" class="channel-submit" on:click={createChannel}>Continue</button>
    </div>
</Modal>

<div class="channels">
    {#if $currentServer}
        <div class="server-title">
            <span />
            <p>{$currentServer?.name}</p>

            {#if $currentServer?.id != "-1"}
                <i class="fa-solid fa-chevron-down options" />
            {/if}
        </div>
    {/if}

    <div class="title">
        <span />
        <p>Channels</p>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i class="fa-solid fa-plus add" on:click={() => creatingChannel = !creatingChannel} />
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
        width: 22%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        border-left: 1px solid rgba(255, 255, 255, 0.1);

        .server-title {
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

            background-color: var(--color-background-highlight);

            span,
            p {
                margin: 0;
                padding: 0;

                text-align: center;
            }

            .options {
                justify-self: right;
                margin-right: 18%;
                cursor: pointer;

                border: none;
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
                text-align: center;
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

    .channel-modal {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        .channel-name {
            width: 80%;
            height: 2rem;
            margin: 4%;

            background: none;
            border: 1px solid #6f7170;
            border-radius: 6px;
            outline: none;
            font-family: "Ubuntu";
            text-indent: 8px;
            color: var(--color-text);
        }

        .channel-submit {
            width: 20%;
            height: 2rem;

            background: none;
            border: 1px solid #6f7170;
            color: var(--color-text);
            border-radius: 8px;
            cursor: pointer;
            outline: none;

            transition: background-color 0.5s ease;

            &:hover {
                background-color: #4f5150;
            }
        }
    }
</style>
