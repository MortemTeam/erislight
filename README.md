# ErisLight
Most powerful optimization using auxtools

## TODO SSlighting

- [ ] lighting_area.dm
- [ ] lighting_atom.dm
- [ ] lighting_corner.dm
- [ ] lighting_overlay.dm
- [ ] lighting_setup.dm
- [ ] lighting_source.dm
- [✓] lighting_turf.dm

## Dependencies

Nerves:

1. Brew coffee

2. Play https://youtu.be/tlqIzNma5KA & https://youtu.be/oMVo2FpbnlM

The [Rust] compiler:

1. Install the Rust compiler's dependencies (primarily the system linker):

   * Ubuntu: `sudo apt-get install gcc-multilib`
   * Windows (MSVC): [Build Tools for Visual Studio 2017][msvc]

1. Use [the Rust installer](https://rustup.rs), or another Rust installation method,
   or run the following:

    ```sh
    curl https://sh.rustup.rs -sSfo rustup-init.sh
    chmod +x rustup-init.sh
    ./rustup-init.sh
    ```

1. Add the **32-bit** compilation target:

    ```sh
    # Clone the `auxtools` repository to a directory of your choice
    git clone https://github.com/MortemTeam/erislight
    # in the `erislight` directory...
    cd erislight
    # Linux
    rustup target add i686-unknown-linux-gnu
    # Windows
    rustup target add i686-pc-windows-msvc
    ```

System libraries:

* Ubuntu and Debian users run:

    ```sh
    sudo dpkg --add-architecture i386
    sudo apt-get update
    sudo apt-get install build-essential g++-multilib libc6-i386 libstdc++6:i386
    ```

* Other Linux distributions install the appropriate **32-bit development** and **32-bit runtime** packages.

## Compiling

The [Cargo] tool handles compilation, as well as automatically downloading and
compiling all Rust dependencies. To compile in release mode (recommended for speed):

Linux:
```sh
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --release --target i686-unknown-linux-gnu
# output: target/i686-unknown-linux-gnu/release/liberislight.so
```

Windows:

```sh
cargo build --release --target i686-pc-windows-msvc
# output: target/i686-pc-windows-msvc/release/erislight.dll
```

[Rust]: https://rust-lang.org
[Cargo]: https://doc.rust-lang.org/cargo
[rustup]: https://rustup.rs
[msvc]: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools

## License

ErisLight is licensed under the [MIT license](https://en.wikipedia.org/wiki/MIT_License).
See [LICENSE](./LICENSE) for more details.
