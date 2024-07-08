# joshka/ratatui-templates

A simple [Ratatui](https://github.com/ratatui-org/ratatui) application template.

## Usage

```shell
cargo install --locked cargo-generate
cargo generate joshka/ratatui-template
```

## Simple-app template

This creates a simple application that creates a [Crossterm] backend, sets up error handling using
[Color-eyre], and runs the App in `app.rs`.

See the [simple-app-generated] folder for full details.

## TODO

- [x] CI Build for checking template
- [ ] Add readme badges to template
- [ ] Add CI workflows to template
- [ ] Add dependabot settings to template
- [ ] Create simple-lib template
- [ ] Create ratatui-calloop based template
- [ ] Create ratatui-bevy based template
- [ ] Create async template

## License

Copyright (c) 2024 Josh McKinney

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](./CONTRIBUTING.md).

[Crossterm]: https://crates.io/crates/crossterm
[Color-eyre]: https://crates.io/crates/color_eyre
[simple-app-generated]: ./simple-app-generated/
