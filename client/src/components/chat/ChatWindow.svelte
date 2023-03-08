<script lang="ts">
    import { fillMessageProps } from "../../api/message";
    import ChatMessage from "./ChatMessage.svelte";
    import { currentChannel, messages } from "../../stores/current";
    import { onDestroy, onMount } from "svelte";
    import { WebSocketAPI } from "../../api/ws";

    $: message = "";

    let messagesRef: HTMLDivElement | undefined;

    const ws = new WebSocketAPI();

    onMount(() => ws.connect());
    onDestroy(() => ws.close());

    let prevChannelId = -1;

    currentChannel.subscribe((c) => {
        if (c && parseInt(c.id) != prevChannelId) {
            prevChannelId = parseInt(c.id);

            ws.getAll();
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
            if (message.replace(/\s/gm, "") == "") return;

            const data = { content: message, timestamp: new Date() };

            ws.send(
                JSON.stringify({
                    action: "SendMessage",
                    data: { ...data, sender: 2, channel: parseInt($currentChannel!.id) },
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
                <ChatMessage {...fillMessageProps(m)} />
            {/each}
        {/if}
    </div>

    {#if $currentChannel}
        <div class="message-input">
            <input
                type="text"
                class="message-input--input"
                placeholder="Type a message..."
                on:keydown={onKeyDown}
                bind:value={message}
            />
        </div>
    {/if}
</div>

<style lang="scss">
    .chat-window {
        height: 94.5%;
        width: calc(96% - (40% + 67px));

        background-color: var(--color-background-light);
        border-left: 1px solid rgba(255, 255, 255, 0.1);

        display: flex;
        flex-direction: column;
        align-items: left;
        justify-content: flex-start;

        padding: 2%;
        padding-top: 1%;

        .messages {
            overflow: scroll;
            height: 92%;
        }

        .message-input {
            height: 6%;
            width: 100%;

            margin-top: 2%;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            &--input {
                height: 90%;
                width: 100%;

                resize: none;

                background-color: var(--color-background-highlight);
                border: 1px solid var(--color-background-highlight);
                border-radius: 8px;
                outline: none;
                text-indent: 2%;
                color: var(--color-text);
                font-family: Verdana, Geneva, Tahoma, sans-serif;
            }
        }
    }
</style>
