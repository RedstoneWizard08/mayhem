<script lang="ts">
    import { getServers } from "../../api/server";
    import { onMount } from "svelte";
    import ServerIcon from "./ServerIcon.svelte";
    import { updateAllChannels } from "../../api/channel";
    import { currentServer, servers, ws } from "../../stores/current";
    import { page } from "$app/stores";
    import Modal from "../Modal.svelte";

    let serverName = "";

    $: creatingServer = false;

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

        $currentServer = $servers.find((s) => s.id == sid) || $currentServer;
    });

    const createServer = () => {
        $ws?.send(
            JSON.stringify({
                action: "CreateServer",

                data: {
                    name: serverName,
                },
            })
        );

        creatingServer = !creatingServer;
        serverName = "";
    };
</script>

<Modal name="Create Server" open={creatingServer} width="40%" height="40%">
    <div class="server-modal">
        <input
            type="text"
            class="server-name"
            placeholder="Server name..."
            bind:value={serverName}
        />

        <button type="button" class="server-submit" on:click={createServer}>Continue</button>
    </div>
</Modal>

<div class="servers">
    <ServerIcon type="channel" id="@me" name="DMs" icon="discord" />

    {#each $servers as server}
        <ServerIcon {...server} />
    {/each}

    <ServerIcon
        type="server"
        id="__"
        name="+"
        icon="add"
        onClick={() => (creatingServer = !creatingServer)}
    />
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

    .server-modal {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        .server-name {
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

        .server-submit {
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
