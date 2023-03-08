<script lang="ts">
    import { currentServer } from "../../stores/current";
    import ChannelIcon from "./ChannelIcon.svelte";
</script>

<div class="channels">
    {#each ($currentServer?.channels || []) as channel}
        <ChannelIcon {...channel} />
    {/each}
</div>

<style lang="scss">
    .channels {
        background-color: var(--color-background-light);

        height: 100%;
        width: 20%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        border-left: 1px solid rgba(255, 255, 255, 0.1);
    }
</style>
