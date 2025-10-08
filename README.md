# Discord Game Activity / Discord Quest Completer

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/markterence/discord-quest-completer/rust-check.yml?branch=main&style=flat&label=build%20artifacts)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/markterence/discord-quest-completer/build-release.yaml?branch=main&style=flat&label=build%20(release))

> Discord Quest Completer. I don't know what to call this, but there it is.

A Windows desktop application for Discord Rich Presence and completing Discord Quest for games without needing to installation the full actual games/applications. Perfect for completing Discord Quests and showing off your gaming status without the storage burden.



<!--
Old attachments: Kept here since github dont provide way to remove/manage attached files.

https://github.com/user-attachments/assets/34ff80c4-9e76-452c-9b02-e56f9ea706dd
https://github.com/user-attachments/assets/de904123-07df-41a9-8db1-ff05cc7ccc9b
-->

https://github.com/user-attachments/assets/de904123-07df-41a9-8db1-ff05cc7ccc9b

---

## 📥 Installation

### Windows

> [!IMPORTANT]
> Make sure you place or extract in a location where you have write or execute permissions.
> 
> The reason for this is that the app will create dummy game file in the same directory.
> By default, the app will not need to be run as administrator, unless if it was installed in a directory that requires elevated permissions. (e.g. `C:\Program Files\`, root of `C:\`, etc.)


> [!NOTE]
> Webview2 is required to run the app. WebView2 comes preinstalled on Windows 11. On versions older than Windows 11 you may need to install it manually.
> If you don't have it installed, you can download it from [here](https://developer.microsoft.com/en-us/microsoft-edge/webview2).


You can download manually pre-built pre-release binaries from the [Releases](https://github.com/markterence/discord-activity/releases) page.

Alternatively, you can follow the [development setup instructions](#development-setup) to build the app from source.

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

---

## ✨ Features

- Simulate playing verified Discord games without installation
- Complete Discord Quests requiring 15-minute gameplay (not yet tested for Stream the game Quests)
- Accessible for users without high-end gaming PCs
- Only Discord Verified games are supported

## ⚙️ How It Works

This app creates small executable files that mimic the actual game processes that Discord looks for when detecting a verified game to use it for it's Rich Presence activity. 

When launched/played, the tiny executables trigger Discord's Rich Presence detection. (As of release build v2025.10.07 the dummy executable file size is 257kb)

## 🛠️ Use Cases

- Complete Discord Quests without downloading massive game files
- Show off playing the latest games to friends
- Save disk space while still participating in Discord's gaming ecosystem
- Useful for users with limited internet bandwidth or storage space

## 🚀 Planned Features and fixes

- Make the "Stop" button work again if process was terminated outside of app's control. (It's already on version 2)
- Persist games that added on the list so it wont reset.
- Discord Activity Simulator/playground (Customizable rich presence for developers and custom activities)
- Set custom activity status from supported games
- Linux and MacOS support (if possible)


## :heart: Support :heart:


[![GitHub][github-badge]][github-sponsors] - Become a Sponsor on GitHub. One time support, or a recurring donation

[![Paypal][paypal-badge]][paypal] - One-time donation via PayPal

---

## 🖥️ Supported Platforms

- Windows 11 (not tested on Windows 10 but it should work)

### 🐧 Linux and 🍎 MacOS Support?

Currently only Windows is supported. But I will try to add Linux support if I can. I don't have a MacOS machine to test on so MacOS support is not gonna happen.

The reason for not having Linux and MacOS support right is that I want to make sure that the dummy game runner have minimal size like around 100KB or less for each platforms. For Windows on example, I used Rust and `windows` crate to use Win32 API to create a dummy window, this compiles to a small 136KB executable. I also tried a C# .net app which is fantastically small its only 7KB but can't make it to where Discord can detect it.

For Linux, I don't know where to start yet. I will try to explore more of what to use and make sure the runner binary is small. It's also a pain (Wine, Proton, etc. How does discord detection work in linux. LMAO same for MacOS). Also the Microsoft Webview2 on Linux or through Wine is somehow problematic to install and get it running.

## 🛠️ Tech Stack

- 🦀 Rust
- 🌐 Vue.js
- 🧰 Tauri

---

## 🧑‍💻 Development Setup

### 🖥️ Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 📋 Requirements

- Tauri - make sure the [pre-requisites](https://tauri.app/v1/guides/getting-started/prerequisites/) are installed such as Rust.
- NodeJS - use any that is latest or node 20+
- pnpm - project uses pnpm as package manager for the frontend.

### 🛠️ Development

Install dependencies for the Vue.js frontend using pnpm

```bash
pnpm install
```

Make sure to build and copy the dummy game binary from `src-win` and is added on tauri application's "resources" folder.

```bash
pnpm build:runner:win && pnpm copy:runner:win
```

Then run the Tauri dev command to start the development server.

```bash
pnpm tauri dev
```

- Also, get the list of detecatable games from the Discord API: `GET /api/applications/detectable` or `GET /api/:version/applications/detectable` and place the JSON file in `src/assets/gamelist.json`


---

## Other Thoughts

### The Discord's RPC server and Rich Presence 

There is also an experimental actions like Discord RPC functionality, it connects to Discord's RPC Gateway to send Activity updates for the selected game using its App ID even if the game is not running. (This may not work for sometime as Discord Updates their RPC and the SDK and syntax that I used on the Rust-code may not be updated as it is not one of the main features).

Though this is functional as it uses Discord Rich Presence, what happens is it uses the AppID and connects it to the RPC. See (https://discord.com/developers/docs/quick-start/getting-started#step-1-creating-an-app)

For example, the AppID for Overwatch is `356875221078245376` and we use it with something like [discordjs/rpc](https://github.com/discordjs/RPC)

```js
// Set this to your Client ID.
const clientId = '356875221078245376'; // But this is Overwatch's AppID on Discord. Not something I created
DiscordRPC.register(clientId);
const rpc = new DiscordRPC.Client({ transport: 'ipc' });
const startTimestamp = new Date();
rpc.setActivity({
  details: `Bleet bleet`,
  state: 'in bleet bleet party',
  startTimestamp,
}); // You will see the Verified Overwatch on Discord Activity with custom details.
```

This may not be the intended use of Discord's RPC and may violate their terms of service. I am not entirely but if you can use other's AppID other than what you had/own in the Discord Developer Dashboard. Use this feature at your own risk.
i

### Disclaimer

This tool is intended for educational purposes and personal use. Please respect Discord's terms of service, partners, game publishers and advertisers rights when using this application.

The creators and maintainers of this project are not liable for any damages, account suspensions, or other consequences that may arise from using this software. Use at your own risk. This project is not affiliated with, endorsed by, or connected to Discord in any way.

---

<!--
## Other Alternatives

If you can't install this application for any reason, there is some steps on a gist from [aamiaa](https://github.com/aamiaa/) that allows you to use Discord client's Web Inspector and paste the code provided and complete the quest.

See the guide here: https://gist.github.com/aamiaa/204cd9d42013ded9faf646fae7f89fbb
-->


## License

[MIT License](LICENSE)© Mark Terence Tiglao - 2025

---


[github-badge]: https://img.shields.io/badge/-Github%20Sponsor-fafbfc?logo=GitHub%20Sponsors
[github-sponsors]: https://github.com/sponsors/markterence
[paypal-badge]: https://img.shields.io/badge/-Paypal-002991?logo=Paypal
[paypal]: https://paypal.me/MarkTerenceTiglao

