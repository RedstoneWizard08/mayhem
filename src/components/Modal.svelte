<script lang="ts">
    export let name: string;
    export let open = false;
</script>

<div class="modal" class:open>
    <div class="inner">
        <div class="bar">
            <span class="name">{name}</span>

            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <span class="fa-solid fa-times close" on:click={() => (open = !open)} />
        </div>

        <slot />
    </div>
</div>

<style lang="scss">
    .modal {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        position: absolute;
        z-index: 4;
        background-color: rgba(0, 0, 0, 0.3);

        opacity: 0;
        pointer-events: none;

        transition: opacity 0.5s ease;

        &.open {
            opacity: 1;
            pointer-events: unset;
        }

        .inner {
            width: 70%;
            height: 70%;

            background-color: #3f4140;

            border-radius: 8px;

            .bar {
                width: 97%;
                height: 7.5%;

                padding: 0 1.5%;

                background-color: #5f6160;

                border-top-left-radius: 8px;
                border-top-right-radius: 8px;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;

                .name {
                    color: #fdfdfd;
                    font-family: Ubuntu;
                }

                .close {
                    cursor: pointer;
                    color: #ededed;
                    font-size: 16pt;

                    transition: color 0.5s ease;

                    &:hover {
                        color: #cdcdcd;
                    }
                }
            }
        }
    }
</style>
