<script lang="ts">
    import type { ChannelIconProps, ServerIconProps } from "src/api/icon";
    import ServerIcon from "./ServerIcon.svelte";

    export let servers: (ServerIconProps | ChannelIconProps)[] = [
        {
            id: "1",
            name: "Test Server 1",
            type: "server",
        },
        {
            id: "2",
            name: "Test Server 2",
            type: "server",
        },
        {
            id: "3",
            name: "Test Server 3",
            type: "server",
        },
        {
            id: "4",
            name: "Test Server 4",
            type: "server",
        },
    ];
</script>

<div class="servers">
    <ServerIcon type="channel" id="@me" name="DMs" icon="discord" />

    {#each servers as server}
        <ServerIcon {...server} />
    {/each}
</div>

<style lang="scss">
    .servers {
        background-color: var(--color-background-light);

        height: 100%;
        width: 65px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;
    }
</style>
