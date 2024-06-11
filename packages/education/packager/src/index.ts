import fs from "fs";
import path from "path";
import * as execa from "execa";
import JSZip from "jszip";
import url from "url";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

const chromeDir = path.join(__dirname, "..", "..", "chrome");
const chromeOutput = path.join(chromeDir, "build");

const token = "mhedu_80r9udhlfky7893ui3";

const buildChrome = async () => {
    console.log("Running build...");

    execa.execaCommandSync("pnpm build", {
        cwd: chromeDir,
    });

    console.log("Creating zip...");

    const zip = new JSZip();

    const assets = zip.folder("assets")!;
    const icons = zip.folder("icons")!;
    const img = zip.folder("img")!;

    zip.file("manifest.json", fs.readFileSync(path.join(chromeOutput, "manifest.json")));
    zip.file(
        "service-worker-loader.js",
        fs.readFileSync(path.join(chromeOutput, "service-worker-loader.js"))
    );

    const assetsDir = path.join(chromeOutput, "assets");
    const iconsDir = path.join(chromeOutput, "icons");
    const imgDir = path.join(chromeOutput, "img");

    for (const item of fs.readdirSync(assetsDir)) {
        assets.file(
            item,
            fs
                .readFileSync(path.join(assetsDir, item))
                .toString()
                .replaceAll("%%_MAYHEM_EDUCATION_TOKEN_%%", token)
        );
    }

    for (const item of fs.readdirSync(iconsDir)) {
        icons.file(item, fs.readFileSync(path.join(iconsDir, item), "base64"), { base64: true });
    }

    for (const item of fs.readdirSync(imgDir)) {
        img.file(item, fs.readFileSync(path.join(imgDir, item), "base64"), { base64: true });
    }

    const file = await zip.generateAsync({ type: "uint8array" });

    fs.writeFileSync("chrome-ext.zip", file);
};

buildChrome();
