<div align="center">
   <img src="https://github.com/Unkn0Wms/UnknProject/blob/main/website/static/assets/logo.png" width="150"></img>
   <h1>UnknProject</h1>
   UnknProject is a simple loader for Counter-Strike game series using the egui library, written entirely in Rust.
</div>

## Showcase

<div align="center">
    <img src="https://github.com/Unkn0Wms/UnknProject/blob/main/website/discord/E50C39DF-1F68-4B01-A5A1-80ED77E754CF.png" width=500>
</div>

## Installation

To install unknproject, you can download [release](https://github.com/unknproject/unknproject/releases/latest), or build it by yourself!

## Usage

<details>
<summary>usage instructions</summary>

To use unknproject you need to download it from [github releases](https://github.com/unknproject/unknproject/releases/latest) or build it by yourself.

Next, you need to run the unknproject executable, and you will see the main window with the list of hacks.

<img src="image/README/1734625752687.png" width=150>

1. Select the hack you want to inject from the list.
2. Click the "Inject" button.
3. Enjoy the game with the hack!

You can also drag and drop the DLL file onto the unknproject window to inject it into the game.

1. Drag and drop the DLL file onto the unknproject window.
2. Select the process you want to inject the DLL into.
3. Enjoy the game with your hack!

</details>

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

### Build Instructions:

1. Clone the repository:

```sh
git clone https://github.com/unknproject/unknproject.git
cd unknproject
```

2. Setup nightly toolchain (for dll-syringe)

```sh
rustup toolchain install nightly
rustup default nightly
```

3. Build the project:

```sh
cargo build --release
```

## Social

-   [Telegram](https://t.me/unkn0wnrage)
-   [Discord](https://discord.gg/Rgx6rnrbAX)
## License

This project is licensed under the GPL 3.0 License. See the [LICENSE](https://github.com/unknproject/unknproject/blob/main/LICENSE) file for details.
