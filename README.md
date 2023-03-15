Forcex2Apic
===========

UEFI driver to enable x2APIC (if the CPU supports it).

#### Build

This project requires a [`x86_64-unknown-uefi`][1] Rust toolchain. This is a
[Tier 2 target][2] starting with Rust 1.68 and [rustup][3] distributes toolchain
binaries for it:

```
rustup target add --toolchain=stable x86_64-unknown-uefi
```

Once the toolchain is installed, use `cargo` to build the project:

```
# Defaults to a debug build
cargo build

# Build in release mode
cargo build --release
```

`cargo` will output `Forcex2Apic.efi` in `target/x86_64-unknown-uefi/debug` or
`target/x86_64-unknown-uefi/release` (if a release build was done).

#### Usage

`Forcex2Apic.efi` is a EFI boot service driver. It can be loaded with the
[`load` command in the EFI shell][4]. For persistence, a driver entry can be
created in NVRAM [using `bcfg driver add`][5].

[1]: https://doc.rust-lang.org/stable/rustc/platform-support/unknown-uefi.html
[2]: https://doc.rust-lang.org/stable/rustc/platform-support.html#tier-2
[3]: https://rust-lang.github.io/rustup/
[4]: https://uefi.org/sites/default/files/resources/UEFI_Shell_2_2.pdf#page=171
[5]: https://uefi.org/sites/default/files/resources/UEFI_Shell_2_2.pdf#page=106
