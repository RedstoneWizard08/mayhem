<script lang="ts">
    import { fillMessageProps } from "../../api/message";
    import ChatMessage from "./ChatMessage.svelte";
    import { messages, currentChannel } from "../../stores/current";

    $: message = "";

    let messagesRef: HTMLDivElement | undefined;

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key == "Enter") {
            if (message.replace(/\s/gm, "") == "") return;

            $messages.push({ content: message, timestamp: new Date() });
            $messages = $messages;

            if ($currentChannel && $currentChannel.type == "channel")
                $currentChannel.messages = $messages;

            message = "";

            setTimeout(() => {
                if (messagesRef)
                    messagesRef.scrollTo({
                        top: messagesRef.scrollHeight,
                        behavior: "smooth",
                    });
            }, 0);
        }
    };
</script>

<div class="chat-window">
    <div class="messages" bind:this={messagesRef}>
        {#each $messages as m}
            <ChatMessage {...fillMessageProps(m)} />
        {/each}
    </div>

    <div class="message-input">
        <input
            type="text"
            class="message-input--input"
            placeholder="Type a message..."
            on:keydown={onKeyDown}
            bind:value={message}
        />
    </div>
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
