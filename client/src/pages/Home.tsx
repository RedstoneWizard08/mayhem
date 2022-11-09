import { route } from "preact-router";
import { useEffect } from "preact/hooks";

export const Home = () => {
    useEffect(() => {
        route("/channels/@me");
    });
    
    return <></>;
};
