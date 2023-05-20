#!/usr/bin/env bun

import { blue, cyan, green, magenta, red, yellow } from "colorette";
import puppeteer from "puppeteer";

const main = async () => {
    const browser = await puppeteer.launch();
    const page = await browser.newPage();

    page.on("console", async (message) => {
        const type = message.type().substr(0, 3).toUpperCase();

        const colors: { [key: string]: (text: string) => string } = {
            LOG: (text) => text,
            ERR: red,
            WAR: yellow,
            INF: cyan,
        };

        const color = colors[type] || blue;

        console.log(color(`${type} ${message.text()}`));

        if (message.text() == "[DEBUG] Attached.") {
            await page.click("canvas", { button: "left" });
        }
    });

    page.on("pageerror", ({ message }) => console.log(red(message)));

    page.on("response", (response) =>
        // !response.status().toString().startsWith("2") &&
        // !response.status().toString().startsWith("3") &&
        console.log(green(`${response.status()} ${response.url()}`))
    );

    page.on("requestfailed", (request) =>
        console.log(magenta(`${request.failure()?.errorText} ${request.url()}`))
    );

    //    page.on("load", async () => {
    //        await page.click("canvas", { button: "left" });
    //    });

    await page.goto(process.argv[2]);

    await new Promise((r) => setTimeout(r, 999999999));

    await browser.close();
};

main();
