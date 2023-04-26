export type BusHandler<T> = (data?: T) => void;

export class EventBus extends EventTarget {
    public constructor() {
        super();
    }

    public dispatch<T>(name: string, data?: T) {
        const event = new CustomEvent(name, { detail: data });

        return this.dispatchEvent(event);
    }

    public on<T>(name: string, handler: BusHandler<T>) {
        this.addEventListener(name, ((ev: CustomEvent) => {
            handler(ev.detail as T | undefined);
        }) as EventListener);
    }
}
