#!/usr/bin/env node
import { spawnSync } from "node:child_process";

const isWindows = process.platform === "win32";
const isDarwin = process.platform === "darwin";

function run(command, args) {
  const result = spawnSync(command, args, { stdio: "inherit", shell: false });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

if (isWindows) {
  run("pnpm", ["run", "build:runner:win"]);
  run("pnpm", ["run", "copy:runner:win"]);
} else if (isDarwin) {
  run("pnpm", ["run", "build:runner:darwin"]);
  run("pnpm", ["run", "copy:runner:darwin"]);
} else {
  console.error(`Unsupported platform for runner sync: ${process.platform}`);
  process.exit(1);
}
