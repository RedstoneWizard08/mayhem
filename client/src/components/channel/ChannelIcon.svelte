<script lang="ts">
    import type { ChannelType } from "../../api/channel";
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { currentChannel, messages as mstore } from "../../stores/current";
    import type { ChatMessageProps } from "../../api/message";

    export let id: string;
    export let name: string;
    export let type: "channel" | "group";
    export let channelType: "text" | "voice" | "announcement" | "events" = "text";
    export let server: string;
    export let channels: ChannelType[] = [];
    export let messages: Partial<ChatMessageProps>[] = [];
    export let hasParent = false;

    $: open = true;
    $: selected = false;

    $: {
        const cid = $page.params.channel;

        if (cid == id) selected = true;
        else selected = false;

        if (type == "group") selected = false;
    }

    $: {
        if (selected) {
            $mstore = messages;
            $currentChannel = $$props as any;
        }
    }

    const handleClick = () => {
        if (type == "channel") goto(`/servers/${server}/channels/${id}`);
        else if (type == "group") open = !open;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
    class="channel"
    class:group={type == "group"}
    class:has-parent={hasParent}
    class:selected
    on:click={handleClick}
>
    {#if type == "channel"}
        {#if channelType == "text"}
            <i class="fa-solid fa-hashtag" />
        {:else if channelType == "voice"}
            <i class="fa-solid fa-microphone" />
        {:else if channelType == "announcement"}
            <i class="fa-solid fa-bullhorn" />
        {:else if channelType == "events"}
            <i class="fa-solid fa-calendar-days" />
        {/if}
    {/if}

    {name}

    {#if type == "group"}
        <i class="fa-solid fa-caret-down" class:fa-caret-down={open} class:fa-caret-up={!open} />
    {/if}
</div>

{#if type == "group" && open}
    {#each channels as channel}
        <svelte:self {...channel} hasParent />
    {/each}
{/if}

<style lang="scss">
    .channel {
        width: 84%;
        margin: 3% 5%;
        padding: 1% 3%;

        border: 1px solid transparent;
        border-radius: 8px;
        background-color: var(--color-background-highlight);

        cursor: pointer;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;

        color: var(--color-text);
        font-size: 12pt;
        font-family: Verdana, Geneva, Tahoma, sans-serif;

        user-select: none;

        transition: border-radius 0.5s ease, border-color 0.5s ease, background-color 0.5s ease;

        &:first-of-type {
            margin-top: 5%;
        }

        &:hover {
            background-color: var(--color-background-highlight-hover);
        }

        &.selected {
            background-color: var(--color-background-selection-light);

            &:hover {
                background-color: var(--color-background-selection-light-hover);
            }
        }

        &.has-parent {
            width: 75%;
            margin-left: 14%;

            i {
                margin-right: 3%;
            }
        }

        &.group {
            background-color: transparent;
            margin-bottom: 0;

            justify-content: space-between;

            &:hover {
                background-color: var(--color-background-highlight);
            }
        }

        i {
            padding: 0;
            margin: 0;
        }
    }
</style>
