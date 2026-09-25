# 09-23-2026
Also starting to think about the ground station. Right now with the outline of the project, It may not be neccesery for me to have a ground station. Although I think if we keep the ground station, the architecture with the ground station will allow multiple clients. not just the tauri app will connect.

# 09-24-2026
Setup my Mac to cross compile the package for raspbery-pi 3.
Install cargo-cross: `cargo install cargo-cross`
Then just built via: `cargo cross build --release --target aarch64-unknown-linux-gnu`
Had to move the TelemetryData Struct into the protocol lib. 
So that the ground-station doesnt bring in the esp-idf lib for compiling to arduino.
Now I need to work on, how the ground station ingests the bytes from the stream to construct packets on the receiving side.
