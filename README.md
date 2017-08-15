## Tool to generate a `dashing.json` for generating docsets for your crate/project

### Installation

The CLI can be installed via:

```bash
cargo install rsdocs-dashing
```

### Usage

Given that you crate is named `<MYCRATE>`:

- Generate the cargo documentation for the crate via `cargo doc` (add feature flags if desired)
- Run `rsdocs-dashing target/doc/<MYCRATE> docset`
- Run `dashing build --config docset/dashing.json --source docset/build`
- Add the newly generate `docset/<MYCRATE>.docset` to your docset viewer


#### Alternatives

Pure Rust approach that does some more heavy lifting: [rsdocs2docset](https://github.com/kesselborn/rsdocs2docset).
Python approach: [rust-docset](https://github.com/vhbit/rust-docset).
