<script lang="ts">
    import ChatMessage from "./ChatMessage.svelte";
    import { currentChannel, messages, user, ws } from "../../stores/current";
    import { onDestroy, onMount } from "svelte";
    import { WebSocketAPI } from "../../api/ws";

    $: message = "";

    let messagesRef: HTMLDivElement | undefined;
    let input: HTMLInputElement | undefined;

    if (!$ws) $ws = new WebSocketAPI();

    onMount(() => $ws?.connect());
    onDestroy(() => $ws?.close());

    let prevChannelId = -1;

    currentChannel.subscribe((c) => {
        if (c && parseInt(c.id) != prevChannelId) {
            prevChannelId = parseInt(c.id);

            $ws?.getAll();
        }
    });

    messages.subscribe(() => {
        setTimeout(() => {
            if (messagesRef)
                messagesRef.scrollTo({
                    top: messagesRef.scrollHeight,
                    behavior: "smooth",
                });
        }, 0);
    });

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key == "Enter") {
            e.preventDefault();
            e.stopPropagation();

            if (e.shiftKey) {
                message += "\n";
                return;
            }

            if (message.replace(/\s/gm, "") == "") return;

            const data = { content: message, timestamp: new Date() };

            $ws?.send(
                JSON.stringify({
                    action: "SendMessage",
                    data: {
                        ...data,
                        sender: $user!.id,
                        channel: parseInt($currentChannel!.id),
                    },
                })
            );

            message = "";
        }
    };
</script>

<div class="chat-window">
    <div class="messages" bind:this={messagesRef}>
        {#if $currentChannel}
            {#each $currentChannel.messages as m}
                <ChatMessage {...m} />
            {/each}
        {/if}
    </div>

    {#if $currentChannel}
        <div class="message-input">
            <input
                type="text"
                class="message-input--input"
                placeholder="Message #{$currentChannel?.name}"
                on:keydown={onKeyDown}
                bind:value={message}
                bind:this={input}
            />
        </div>
    {/if}
</div>

<style lang="scss">
    .chat-window {
        height: 100%;
        width: calc(94% - (40% + 67px));

        background-color: var(--color-background-light);
        border-left: 1px solid rgba(255, 255, 255, 0.1);

        display: flex;
        flex-direction: column;
        align-items: left;
        justify-content: space-between;

        padding: 0 2%;

        .messages {
            overflow: scroll;
            height: calc(100% - 6%);
        }

        .message-input {
            height: 6%;
            width: 100%;

            margin: 2% 0;
            padding: 0;

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            &--input {
                height: 100%;
                width: 100%;

                resize: none;
                margin: 0;
                padding: 0;

                background-color: var(--color-background-highlight);
                border: 1px solid var(--color-background-highlight);
                border-radius: 8px;
                outline: none;
                text-indent: 2%;
                color: var(--color-text);
                font-family: "Ubuntu";
            }
        }
    }
</style>
