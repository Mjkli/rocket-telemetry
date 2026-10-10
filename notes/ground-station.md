# 09-23-2026
Also starting to think about the ground station. Right now with the outline of the project, It may not be neccesery for me to have a ground station. Although I think if we keep the ground station, the architecture with the ground station will allow multiple clients. not just the tauri app will connect.

# 09-24-2026
Setup my Mac to cross compile the package for raspbery-pi 3.
Install cargo-cross: `cargo install cargo-cross`
Then just built via: `cargo cross build --release --target aarch64-unknown-linux-gnu`
Had to move the TelemetryData Struct into the protocol lib. 
So that the ground-station doesnt bring in the esp-idf lib for compiling to arduino.
Now I need to work on, how the ground station ingests the bytes from the stream to construct packets on the receiving side.

# 09-26-2026
figured out some issues with my raspberry-pi. Have the serial port open now and is reading the UART stream properly now.
Just have to build in the stream parcer / buffer to get packets out of the stream to read.

# 10-10-2026
Okay getting back to this. Was studying leetcode problems. But going to pick up where i left off, building out the packet buffer.
Intial feeling is building out a circular buffer that can hold multiple packets.
Okay working into this, I am figuring out that I cant really parse the stream unless i give myself some keys to look at. header / footer.
So I can look for this pattern in the stream and construct the packet from there.
Added the parsing now. And working in a cargo test. Need to rn a test from the flight computer to ground-station now. 
But I did notice I am not passing the packet count with the packet. So I need to go back and add that so I can get it later.
Added count to the telemetry Data. So that I dont have to worry about extracting after the fact. 
Logic is able to parse a stream of u8s and return a vec of packets. Need to add more logic to test what happens if the vec buffer over fills.