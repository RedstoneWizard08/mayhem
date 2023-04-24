<script lang="ts">
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";
    import { createAccount, getToken } from "../../api/auth";

    $: firstName = "";
    $: lastName = "";
    $: email = "";
    $: username = "";
    $: password = "";

    $: register = true;

    const submit = async (ev: SubmitEvent) => {
        ev.preventDefault();
        ev.stopPropagation();

        let result: string;

        if (register) {
            result = await createAccount(firstName, lastName, email, username, password);
        } else {
            result = await getToken(username, password);
        }

        goto("/", {
            invalidateAll: true,
        });

        if (browser) {
            localStorage.setItem("__MAYHEM_TOKEN__", result);

            location.replace("/");
        }
    };
</script>

<div class="login-form">
    <form autocomplete="off" method="post" action="" class="form" on:submit={submit}>
        <span class="title">
            Sign {register ? "up for" : "in to"} Mayhem
        </span>

        {#if register}
            <input
                type="text"
                class="input"
                name="firstname"
                autocomplete="off"
                placeholder="First name"
                bind:value={firstName}
            />

            <input
                type="text"
                class="input"
                name="lastname"
                autocomplete="off"
                placeholder="Last name"
                bind:value={lastName}
            />

            <input
                type="email"
                class="input"
                name="email"
                autocomplete="off"
                placeholder="Email"
                bind:value={email}
            />
        {/if}

        <input
            type="text"
            class="input"
            name="username"
            autocomplete="off"
            placeholder="Username"
            bind:value={username}
        />

        <input
            type="password"
            class="input"
            name="password"
            autocomplete="off"
            placeholder="Password"
            bind:value={password}
        />

        <button type="submit" class="submit">Continue</button>

        <button type="button" class="switch" on:click={() => (register = !register)}>
            {register ? "Already have an account? Sign in!" : "No account? Sign up!"}
        </button>
    </form>
</div>

<style lang="scss">
    .login-form {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        .form {
            width: 30%;
            height: 70%;

            border-radius: 8px;
            background-color: var(--color-background-highlight);

            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            .title {
                font-family: Ubuntu;
                color: var(--color-text);
                font-size: 16pt;
                margin: 4%;
            }

            .input {
                height: 6%;
                width: 80%;
                border-radius: 8px;
                background: none;
                border: 1px solid #6f7170;
                outline: none;
                margin: 2%;
                color: white;
                text-indent: 2%;

                transition: border-color 0.5s ease;

                &:hover,
                &:focus {
                    border-color: #8f9190;
                }

                &:-webkit-autofill,
                &:-webkit-autofill:hover,
                &:-webkit-autofill:focus,
                &:-webkit-autofill:active {
                    background: none;
                    -webkit-box-shadow: 0 0 0px 1000px rgba(0, 0, 0, 0.1) inset;
                }
            }

            .submit {
                margin: 2%;
                height: 6%;
                width: 30%;
                border-radius: 8px;
                border: 1px solid #6f7170;
                background-color: #6f7170;
                color: #efefef;
                font-family: Ubuntu;
                cursor: pointer;
                font-size: 12pt;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #4f5150;
                }
            }

            .switch {
                background: none;
                border: 1px solid transparent;
                color: #fcfefc;
                font-size: 12pt;
                margin: 2%;
                cursor: pointer;
                outline: none;

                transition: border-bottom-color 0.5s ease;

                &:hover {
                    border-bottom-color: #fdfdfd;
                }
            }
        }
    }
</style>
