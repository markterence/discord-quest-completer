# Discord Game Activity / Discord Rich Presence

> Discord Quest Completer, Discord Rich Presence Simulator whatever. I don't know what to call this.

A desktop application that simulates Discord Rich Presence for games without requiring installation of the actual games. Perfect for completing Discord Quests and showing off your gaming status (discord kids gonna love it) without the storage burden.

<!--
Old attachments: Kept here since github dont provide way to remove/manage attached files.

https://github.com/user-attachments/assets/34ff80c4-9e76-452c-9b02-e56f9ea706dd
https://github.com/user-attachments/assets/de904123-07df-41a9-8db1-ff05cc7ccc9b
-->

https://github.com/user-attachments/assets/de904123-07df-41a9-8db1-ff05cc7ccc9b

## Installation

### Windows

> Webview2 is required to run the app. WebView2 comes preinstalled on Windows 11. On versions older than Windows 11 you may need to install it manually.
> If you don't have it installed, you can download it from [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2).

You can run follow the [development setup instructions](#Development Setup) to build the app from source.

Alternatively, you can download manually pre-built pre-release binaries from the [Releases](https://github.com/markterence/discord-activity/releases) page.

- [discord-quest-completer-v.0.1.0-pre-release](https://github.com/markterence/discord-activity/commits/v0.1.0-pre-release)

## Uninstall

To uninstall the app, simply go to the folder where you extracted the app and delete the contents folder.

The content of the folder may look like this:

```text
folder-name/
├── discord-quest-completer.exe (main app)
├── data/ 
│   ├── src-win.exe (runner dummy template)
├── games/
│    ├── <game-id>/
```

## Features

- Simulate playing verified Discord games without installation
- Complete Discord Quests requiring 15-minute gameplay (not yet tested for Stream the game Quests)
- Accessible for users without high-end gaming PCs
- Only Discord Verified games are supported

## How It Works

This app creates small executable files that mimic the actual game processes that Discord looks for when detecting a verified game to use it for it's Rich Presence activity. When launched/played, the tiny executables trigger Discord's Rich Presence detection while consuming minimal system resources.

There is also an experimental discord RPC functionality, it connects to Discord's RPC Gateway to send Activity updates for the selected game using its App ID even if the game is not running. Though this is functional, this may not be the intended use of Discord's RPC and may violate their terms of service (may be under _self-botting_) and can put your Discord account at risk. Use this feature at your own risk.

## Use Cases

- Complete Discord Quests without downloading massive game files
- Show off playing the latest games to friends
- Save disk space while still participating in Discord's gaming ecosystem
- Useful for users with limited internet bandwidth or storage space

## Planned Features and fixes

- Make the "Stop" button work again if process was terminated outside of app's control.
- Save added games to a list for easy access
- Uninstall / Clean the installed dummy game runners.
- Discord Activity Simulator/playground (Customize and preview rich presence for developers and custom activities)
- Set custom activity status from supported games
- Linux and MacOS support (if possible)


## Supported Platforms

- Windows 11 (not yet tested on Windows 10 but it should work)

## Linux and MacOS support ?

Currently only Windows is supported. But I will try to add Linux support. I don't have a MacOS machine to test on so MacOS support is not gonna happen.

The reason for not having Linux and MacOS support right is that I want to make sure that the dummy game runner have minimal size like around 100KB or less for each platforms. For Windows on example, I used Rust and `windows` crate to use Win32 API to create a dummy window, this compiles to a small 136KB executable. I also tried a C# .net app which is fantastically small its only 7KB but can't make it to where Discord can detect it.

For Linux, I don't know where to start yet. I will try to explore more of what to use and make sure the runner binary is small and it's a pain (Wine, Proton, etc. How does discord detection work in linux. LMAO same for MacOS)

## TechStack

- Tauri
- Rust
- Vue.js

## Development Setup

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Requirements

- Tauri - make sure the [pre-requisites](https://tauri.app/v1/guides/getting-started/prerequisites/) are installed such as Rust.
- NodeJS - use any that is latest or node 20+
- pnpm - project uses pnpm as package manager for the frontend.

### Development

Install dependencies for the Vue.js frontend using pnpm

```bash
pnpm install
```

Make sure to build and copy the dummy game binary from `src-win` and is added on tauri's resources folder.

```bash
pnpm build:runner:win && pnpm copy:runner:win
```

Then run the Tauri dev command to start the development server.

```bash
pnpm tauri dev
```

- Also, get the list of detecatable games from the Discord API: `GET /api/applications/detectable` or `GET /api/:version/applications/detectable` and place the JSON file in `src/assets/gamelist.json`

## Disclaimer

This tool is intended for educational purposes and personal use. Please respect Discord's terms of service and game publishers' rights when using this application.

The creators and maintainers of this project are not liable for any damages, account suspensions, or other consequences that may arise from using this software. Use at your own risk. This project is not affiliated with, endorsed by, or connected to Discord in any way.

## License

[MIT License](LICENSE)© Mark Terence Tiglao - 2025
