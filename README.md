# Bare Metal OS
Following os.phil-opp.com. A simple kernel for x86_64.

### Dependencies on MacOS
```
cargo install cargo-xbuild
rustup component add rust-src
cargo install bootloader
rustup component add llvm-tools-preview
cargo bootimage
brew install qemu
```

### Run
```
cd os/ && cargo xrun
```