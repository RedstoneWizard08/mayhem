<script lang="ts">
    import { getServers } from "../../api/server";
    import { onMount } from "svelte";
    import ServerIcon from "./ServerIcon.svelte";
    import { updateAllChannels } from "../../api/channel";
    import { currentServer, servers } from "../../stores/current";
    import { page } from "$app/stores";

    onMount(async () => {
        const resp = await getServers();

        $servers = resp.servers.map((v) => ({
            id: v.id.toString(),
            name: v.name,
            type: "server",
            channels: [],
        }));

        await updateAllChannels();

        const sid = $page.params.server;

        $currentServer = $servers.find((s) => s.id == sid) || $currentServer || $servers[0];
    });
</script>

<div class="servers">
    <ServerIcon type="channel" id="@me" name="DMs" icon="discord" />

    {#each $servers as server}
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
