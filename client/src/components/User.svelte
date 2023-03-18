<script lang="ts">
    import { user } from "../stores/current";

    $: username = $user?.username;
    $: avatar = "https://ui-avatars.com/api/?background=random&rounded=true&name=" + username;

    $: muted = false;
    $: deafened = false;
</script>

<div class="user-info">
    <img src={avatar} alt={username} class="avatar" />

    <div class="text">
        <p class="username">@{username}</p>
        <p class="tag">#0000</p>
    </div>

    <div class="buttons">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i
            class="fa-solid"
            class:fa-microphone={!muted}
            class:fa-microphone-slash={muted}
            on:click={() => (muted = !muted)}
        />

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <i
            class="fa-solid"
            class:fa-phone={!deafened}
            class:fa-phone-slash={deafened}
            on:click={() => (deafened = !deafened)}
        />
        
        <i class="fa-solid fa-gear" />
    </div>
</div>

<style lang="scss">
    .user-info {
        width: 100%;
        height: 6%;

        border-top: 1px solid rgba(255, 255, 255, 0.1);

        margin-top: auto;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;

        .avatar {
            height: 75%;
            margin-left: 3%;
            user-select: none;
        }

        .text {
            flex-grow: 1;
            height: 100%;
            color: white;

            font-size: 10.5pt;
            font-family: "Ubuntu";

            margin-left: 4%;

            display: flex;
            flex-direction: column;
            align-items: flex-start;
            justify-content: center;

            p {
                margin: 0;
                padding: 0;
                user-select: none;
            }

            .username {
                cursor: pointer;
                font-weight: 600;

                text-decoration: underline;
                text-decoration-color: transparent;

                transition: text-decoration-color 0.5s ease;

                &:hover {
                    text-decoration-color: white;
                }
            }

            .tag {
                color: #acadac;
                font-size: 9.5pt;
            }
        }

        .buttons {
            padding: 0 1%;
            margin-right: 3%;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;

            i {
                color: white;
                width: 1.25rem;
                cursor: pointer;
                margin: 2%;
                padding: 0.3rem;
                border-radius: 4px;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                transition: background-color 0.5s ease;

                &:last-of-type {
                    margin-right: 0;
                }

                &:hover {
                    background-color: #4d4d4d;
                }

                &.fa-microphone-slash, &.fa-phone-slash {
                    color: #cc1d1c;
                }
            }
        }
    }
</style>
