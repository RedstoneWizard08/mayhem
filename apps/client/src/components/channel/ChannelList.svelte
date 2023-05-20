<script lang="ts">
    import { onMount } from "svelte";
    import { leaveServer, deleteServer } from "../../api/ws/server";
    import { currentServer, ws, user } from "../../stores/current";
    import { isBlank } from "../../util";
    import Modal from "../Modal.svelte";
    import User from "../User.svelte";
    import ChannelIcon from "./ChannelIcon.svelte";
    import { browser } from "$app/environment";

    let channelName = "";
    let dropdown: HTMLDivElement;
    let trigger: HTMLElement;

    $: actions = false;
    $: creatingChannel = false;

    const createChannel = async () => {
        if (isBlank(channelName)) return;

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

    onMount(() => {
        if (browser) {
            window.addEventListener("keydown", (ev) => {
                if (ev.code == "Escape") actions = false;
            });

            window.addEventListener("click", (ev) => {
                const tgt = ev.target as HTMLElement;
                const parent = tgt.parentElement;
                const parent2 = parent?.parentElement;

                if (parent != dropdown && parent2 != dropdown) actions = false;
            });
        }
    });

    const onLeaveClick = async () => {
        if ($currentServer && $user && $ws && $ws.get()) {
            await leaveServer($user.id, parseInt($currentServer.id), $ws.get()!);
        }

        actions = false;
    };

    const onDeleteClick = async () => {
        if ($currentServer && $ws && $ws.get())
            await deleteServer(parseInt($currentServer.id), $ws?.get()!);

        actions = false;
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

            <div class="options" bind:this={dropdown}>
                {#if $currentServer?.id != "-1"}
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <i
                        bind:this={trigger}
                        class="fa-solid fa-chevron-down trigger"
                        class:open={actions}
                        on:click={() => (actions = !actions)}
                    />
                {/if}

                <div class="content" class:open={actions}>
                    <span class="title">Options</span>
                    <hr class="divider" />

                    <button type="button" class="action red" on:click={onLeaveClick}>
                        <i class="fa-solid fa-right-from-bracket" />
                        Leave Server
                    </button>

                    <button type="button" class="action red" on:click={onDeleteClick}>
                        <i class="fa-solid fa-trash-can" />
                        Delete Server
                    </button>
                </div>
            </div>
        </div>
    {/if}

    <div class="title">
        <span />
        <p>Channels</p>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i class="fa-solid fa-plus add" on:click={() => (creatingChannel = !creatingChannel)} />
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

                display: inline-flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                position: relative;

                .trigger {
                    cursor: pointer;

                    border: none;
                    border-radius: 4px;
                    padding: 0.5rem;

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    text-align: center;

                    transition: background-color 0.5s ease;

                    &:hover {
                        background-color: #2f3130;
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

                    &::before {
                        transition: transform 0.5s ease;
                    }

                    &.open {
                        &::before {
                            transform: rotate(-180deg);
                        }
                    }
                }

                .content {
                    top: 1.75rem;
                    display: block;
                    position: absolute;
                    background-color: #2f3130;
                    border: 1px solid rgba(255, 255, 255, 0.2);
                    padding: 0.5rem;
                    z-index: 1;
                    border-radius: 4px;

                    opacity: 0;
                    transform: scaleY(0);
                    transform-origin: top;

                    pointer-events: none;

                    transition: opacity 0.5s ease, transform 0.25s ease;

                    &.open {
                        opacity: 1;
                        transform: scaleY(1);
                        pointer-events: unset;
                    }

                    .title {
                        font-size: 12pt;
                        padding: 0 2.2rem;
                    }

                    .divider {
                        border: none;
                        border-bottom: 1px solid #9c9c9c;
                        width: 100%;
                    }

                    .action {
                        width: 100%;
                        background-color: transparent;
                        border: none;
                        border-radius: 4px;

                        color: #fdfdfd;
                        font-family: Ubuntu;

                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: space-between;

                        padding: 0.5rem 0.6rem;
                        margin: 0.4rem 0;

                        cursor: pointer;
                        outline: none;

                        transition: background-color 0.25s ease;

                        &:hover {
                            background-color: #4f5150;
                        }

                        &:last-child {
                            margin: 0;
                        }

                        &.red {
                            background-color: rgba(210, 32, 32, 0.4);

                            &:hover {
                                background-color: rgba(210, 32, 32, 0.8);
                            }
                        }
                    }
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
