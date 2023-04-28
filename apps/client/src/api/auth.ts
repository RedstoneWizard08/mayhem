import { page } from "$app/stores";
import axios from "axios";
import { get } from "svelte/store";
import { token, user } from "../stores/current";
import type { UserInfo } from "./user";

export const login = async (username: string, password: string) => {
    const uri = new URL("/api/users", get(page).url.href);

    const resp = await axios.post<UserInfo>(uri.toString(), {
        username,
        password,
    });

    return resp.data;
};

export const loginWithToken = async (token: string) => {
    const uri = new URL("/api/users/token", get(page).url.href);

    const resp = await axios.post<UserInfo>(uri.toString(), {
        token,
    });

    return resp.data;
};

export const getToken = async (username: string, password: string) => {
    const uri = new URL("/api/token", get(page).url.href);

    const resp = await axios.post<string>(uri.toString(), {
        username,
        password,
    });

    return resp.data;
};

export const authenticate = async (username: string, password: string) => {
    const userInfo = await login(username, password);
    const tokenGet = await getToken(username, password);

    user.set(userInfo);
    token.set(tokenGet);
};

export const createAccount = async (
    firstName: string,
    lastName: string,
    email: string,
    username: string,
    password: string
) => {
    const uri = new URL("/api/users", get(page).url.href);

    const resp = await axios.put<UserInfo>(uri.toString(), {
        first_name: firstName,
        last_name: lastName,
        username,
        email,
        password,
    });

    user.set(resp.data);

    const token = await getToken(username, password);

    return token;
};
