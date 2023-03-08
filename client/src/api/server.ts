import axios from "axios";
import { buildRequest, ROUTES } from "./constants";

export const getServerInfo = async () => {
    const config = buildRequest(ROUTES.servers.info);
    const result = await axios.request(config);

    console.log(result);
};
