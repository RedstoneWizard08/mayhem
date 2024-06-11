import chalk from "chalk";
import { ChildProcess, execSync, spawn, StdioOptions } from "child_process";
import { runProcess } from "./run-process";

interface CargoRun {
    success: boolean;
    output: string;
}

interface RunCargoOptions {
    stdio: StdioOptions;
    env: NodeJS.ProcessEnv | undefined;
}

export let childProcess: ChildProcess | null;

export async function cargoCommand(
    cross?: boolean,
    ...args: string[]
): Promise<{ success: boolean }> {
    console.log(chalk.dim(`> ${cross ? "cross" : "cargo"} ${args.join(" ")}`));
    args.push("--color", "always");
    return runProcess(cross ? "cross" : "cargo", ...args);
}

export function cargoRunCommand(cross?: boolean, ...args: string[]): Promise<{ success: boolean }> {
    console.log(chalk.dim(`> ${cross ? "cross" : "cargo"} ${args.join(" ")}`));
    return new Promise((resolve, reject) => {
        childProcess = spawn(cross ? "cross" : "cargo", [...args, "--color", "always"], {
            cwd: process.cwd(),
            windowsHide: true,
            detached: true,
            shell: false,
            stdio: ["inherit", "inherit", "inherit"],
        });

        // Ensure the child process is killed when the parent exits
        process.on("exit", () => childProcess?.kill());
        process.on("SIGTERM", () => childProcess?.kill());
        process.on("SIGINT", () => childProcess?.kill());

        childProcess.on("error", (err) => {
            reject({ success: false });
        });

        childProcess.on("exit", (code) => {
            childProcess = null;
            if (code === 0) {
                resolve({ success: true });
            } else {
                reject({ success: false });
            }
        });
    });
}

export function cargoCommandSync(
    cross?: boolean,
    args = "",
    options?: Partial<RunCargoOptions>
): CargoRun {
    const normalizedOptions: RunCargoOptions = {
        stdio: options?.stdio ?? "inherit",
        env: {
            ...process.env,
            ...options?.env,
        },
    };

    try {
        return {
            output: execSync(`${cross ? "cross" : "cargo"} ${args}`, {
                encoding: "utf8",
                windowsHide: true,
                stdio: normalizedOptions.stdio,
                env: normalizedOptions.env,
            }),
            success: true,
        };
    } catch (e) {
        return {
            output: e as string,
            success: false,
        };
    }
}
