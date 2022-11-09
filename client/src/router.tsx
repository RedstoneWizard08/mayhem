import { Route, Router } from "preact-router";
import { Home } from "./pages/Home";
import { PrivateChannel } from "./pages/PrivateChannel";
import { ServerChannel } from "./pages/ServerChannel";

export const AppRouter = () => {
    return (
        <Router>
            <Route path="/" component={Home} />

            <Route path="/channels/:channel" component={PrivateChannel} />

            <Route path="/servers/:server" component={ServerChannel} />
            <Route path="/servers/:server/channels/:channel" component={ServerChannel} />
        </Router>
    );
};
