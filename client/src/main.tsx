import "@fortawesome/fontawesome-free/scss/fontawesome.scss";
import "@fortawesome/fontawesome-free/scss/regular.scss";
import "@fortawesome/fontawesome-free/scss/solid.scss";
import "@fortawesome/fontawesome-free/scss/brands.scss";

import "./styles/main.scss";

import { render } from "preact";
import { AppRouter } from "./router";

const root = document.getElementById("root") as HTMLElement;

render(
    <AppRouter />,
    root,
);
