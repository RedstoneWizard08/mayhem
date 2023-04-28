<script lang="ts">
    import "../styles/main.scss";

    import ChannelList from "../components/channel/ChannelList.svelte";
    import ChatWindow from "../components/chat/ChatWindow.svelte";
    import MemberList from "../components/member/MemberList.svelte";
    import ServerList from "../components/server/ServerList.svelte";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import { token, user } from "../stores/current";
    import { loginWithToken } from "../api/auth";
    import { goto } from "$app/navigation";
    import type { UserInfo } from "../api/user";
    import { page } from "$app/stores";

    $: loading = true;
    $: login = false;

    onMount(async () => {
        if ($page.url.pathname == "/login") {
            login = true;
            loading = false;
            return;
        }

        if (browser) {
            const localToken = localStorage.getItem("__MAYHEM_TOKEN__");

            if (localToken) {
                let userData: UserInfo | null = null;

                try {
                    userData = await loginWithToken(localToken);
                } catch (e) {
                    goto("/login");
                    login = true;
                    loading = false;
                    return;
                }

                $user = userData;
                $token = localToken;
            } else {
                goto("/login");

                login = true;
            }
        }

        loading = false;
    });
</script>

{#if loading}
    <div class="loader">
        <p>Loading...</p>

        <span class="icon" />
    </div>
{:else if !login}
    <div class="app">
        <ServerList />
        <ChannelList />
        <ChatWindow />
        <MemberList />
        <slot />
    </div>
{:else}
    <slot />
{/if}

<style lang="scss">
    .app {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: row;
        align-items: flex-start;
        justify-content: flex-start;
    }

    .loader {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        p {
            color: white;
            font-family: "Ubuntu";

            font-size: 16pt;
            text-align: center;

            margin: 2%;
        }

        .icon {
            display: inline-block;
            width: 60px;
            height: 60px;

            &:after {
                content: " ";
                display: block;
                width: 48px;
                height: 48px;
                margin: 0;

                border: 3px solid transparent;
                border-left-color: #fff;
                border-radius: 50%;

                animation: loader 1.2s linear infinite;
            }
        }
    }

    @keyframes loader {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>
