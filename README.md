# neovim-client
[Neovim-rs](https://github.com/daa84/neovim-lib) fork. Rust library for Neovim msgpack-rpc clients.

## Motivation 
I am in the process of writing a neovim plugin with rust, and decided to use this lib. After a period of 
confusion and research, I discovered that this library can support most of neovim latest apis (via codegen).
It is just not obvious, and the latest release on crates.io being almost two years old adds to the confusion.

The documentation is also in dire need of love.

## Disclaimer
I know almost nothing about neovim api, rust codegen, and the code in this repository. This is mostly to learn
and help a few other lost souls on this dark path. We should be able to write rust neovim plugins without losing our sanity.

I will try to document the code a bit while working with it. 

Any contribution is more than welcome !

## Codegen
If you clone the repo, you can generate the rust code againt your current neovim install with this command

```bash
make generate
```

## Useful links
* [Neovim Api Documentation](https://neovim.io/doc/user/api.html)
* [Writing Neovim plugins in Rust](https://blog.usejournal.com/a-detailed-guide-to-writing-your-first-neovim-plugin-in-rust-a81604c606b1)
* [Proof-of-concept for a Neovim plugin written in Rust](https://github.com/boxofrox/neovim-scorched-earth)
