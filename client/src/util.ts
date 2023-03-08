export const trim = (str: string, length: number) => {
    if (str.length <= length) return str;
    else return str.substring(0, length - 3) + "...";
};
