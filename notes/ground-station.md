# 09-23-2026
Also starting to think about the ground station. Right now with the outline of the project, It may not be neccesery for me to have a ground station. Although I think if we keep the ground station, the architecture with the ground station will allow multiple clients. not just the tauri app will connect.

# 09-24-2026
Setup my Mac to cross compile the package for raspbery-pi 3.
Install cargo-cross: `cargo install cargo-cross`
Then just built via: `cargo cross build --release --target aarch64-unknown-linux-gnu`