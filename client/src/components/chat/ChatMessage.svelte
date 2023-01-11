<script lang="ts">
    import { onMount } from "svelte";
    import moment from "moment";
    import { fixContent } from "../../api/message";

    export let avatar: string;
    export let name: string;
    export let timestamp: string | Date;
    export let content: string;

    $: realContent = fixContent(content);
    $: time = moment(timestamp).fromNow();

    onMount(() => {
        setInterval(() => {
            time = moment(timestamp).fromNow();
        }, 1000);
    });
</script>

<div class="chat-message">
    <img src={avatar} alt={name} />

    <div class="content">
        <div class="info">
            <p class="author">{name}</p>
            <p class="timestamp">{time}</p>
        </div>

        <p class="message">{@html realContent}</p>
    </div>
</div>

<style lang="scss">
    .chat-message {
        display: flex;
        flex-direction: row;
        align-items: flex-start;
        justify-content: flex-start;

        &:not(:last-of-type) {
            margin-bottom: 1%;
        }

        .content {
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            justify-content: flex-start;
            font-family: Verdana, Geneva, Tahoma, sans-serif;
            height: 100%;

            .info {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: flex-start;
                margin-bottom: 6px;
                user-select: none;

                .author {
                    color: var(--color-text);
                    margin-right: 10px;
                    cursor: pointer;
                    font-weight: bold;

                    &:hover {
                        text-decoration: underline;
                    }
                }

                .timestamp {
                    color: var(--color-text-light);
                }

                p {
                    margin-bottom: 0;
                }
            }

            .message {
                margin-top: 0;
                color: var(--color-text);
            }
        }

        img {
            height: 50px;
            width: 50px;
            border-radius: 50%;

            margin-right: 2%;
            user-select: none;
            cursor: pointer;

            margin-top: 2.5%;
        }
    }
</style>
