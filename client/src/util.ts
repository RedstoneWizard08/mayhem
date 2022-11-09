import { JSX } from "preact/jsx-runtime";

export interface DefaultComponentProps {
    children?: JSX.Element | JSX.Element[];
};

export const trim = (str: string, length: number) => {
    if (str.length <= length) return str;
    else {
        let out = "";
        for (let i = 0; i < length; i++) out += str.charAt(i);
        return out;
    }
}
