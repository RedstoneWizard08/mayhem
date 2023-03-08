<script lang="ts">
    import "../styles/main.scss";

    import ChannelList from "../components/channel/ChannelList.svelte";
    import ChatWindow from "../components/chat/ChatWindow.svelte";
    import MemberList from "../components/member/MemberList.svelte";
    import ServerList from "../components/server/ServerList.svelte";
</script>

<div class="app">
    <ServerList />
    <ChannelList />
    <ChatWindow />
    <MemberList />
    <slot />
</div>

<style lang="scss">
    .app {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: row;
        align-items: flex-start;
        justify-content: flex-start;
    }
</style>
