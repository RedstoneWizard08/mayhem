import { page } from "$app/stores";
import axios from "axios";
import { get } from "svelte/store";
import type { UserInfo } from "./user";

export interface HasId<T> {
    id: T;
}

// export type ResolveTask<T> = Promise<T>;

export class ResolveTask<T> extends EventTarget {
    private _promise: Promise<T>;
    private _value: T | Error;
    private _done: boolean;

    public constructor(promise: Promise<T>) {
        super();

        this._promise = promise;
        this._done = false;

        this._init();
    }

    private _init() {
        this._promise.then(this._onComplete.bind(this));
        this._promise.catch(this._onError.bind(this));
    }

    private _onComplete(value: T) {
        this._done = true;
        this._value = value;

        this.dispatchEvent(new CustomEvent("resolve", { detail: this._value }));
    }

    private _onError(error: Error) {
        this._done = true;
        this._value = error;

        this.dispatchEvent(new CustomEvent("error", { detail: this._value }));
    }

    public inner() {
        return this._promise;
    }

    public get() {
        if (this._done) return this._value;

        return null;
    }
}

export abstract class Cache<I, T extends HasId<I>> {
    protected entries: Map<I, T>;
    protected queue: ResolveTask<T | undefined>[];

    public constructor() {
        this.entries = new Map();
        this.queue = [];
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    protected enqueue(promise: Promise<T | undefined>) {
        const task = new ResolveTask(promise);

        task.addEventListener("complete", this._resolved.bind(this) as EventListener);
        task.addEventListener("error", this._resolved.bind(this) as EventListener);

        this.queue.push(task);
    }

    private _resolved(ev: CustomEvent) {
        const data = ev.detail;

        if (data instanceof Error) console.error(data);
        else this.set(data as T);
    }

    public abstract resolve(id: I): Promise<T | undefined>;

    public set(item: T): void {
        this.entries.set(item.id, item);
    }

    private async _resolve(id: I): Promise<T | undefined> {
        if (!id) return undefined;

        const res = await this.resolve(id);

        if (res) this.set(res);

        return res;
    }

    public async get(id: I, refresh = true): Promise<T | undefined> {
        if (!id) return undefined;

        const result = this.entries.get(id);

        if (!result) return await this._resolve(id);

        if (refresh) this.enqueue(this.resolve(id));

        return result;
    }
}

export class UserCache extends Cache<number, UserInfo> {
    public async resolve(id: number): Promise<UserInfo | undefined> {
        if (!id) return undefined;

        const uri = new URL("/api/users/" + id, get(page).url.href);

        const resp = await axios.get<UserInfo>(uri.toString());

        return resp.data;
    }
}
