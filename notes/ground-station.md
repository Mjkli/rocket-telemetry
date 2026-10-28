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

# 10-11-2026
I added a test that adds noise to the packet chunk. To Simulate "static" this would break the flow of a good packet stream adding in more u8s then wanted in a packet. 
Im not sure If we should tackle this edge case now, and just move forward with trying to get packets flowing from the flight computer.
Okay ran into another issue. I need to re-think how I handle the buffer logic. 
Okay I found out that i am querying the bus to fast and doing looping operations for only 1 u8 at at time. I backed off the loop speed, and made the speed variable to the amount of buffers coming in.
Buffer logic seems to be working now, Now I think I need to return to the packet parsing logic. As If I dont have the proper logic,
I get an infinate loop. Lookng for the first 0x00 and then assuming the next 39 bytes are the packet isnt correct. I think I need to
keep a pointer at the 0x00 and tehn scan ahead to find the next 0x00. That should tell me the bounds of the packet.
Okay Got IT! I had to update the parsing loop to only do an iter to the first 0x00 and then another loop that starts from that position,
and find the next 0x00. 
After i did this and tweaked the timing on the buffer, I am able to see the telemetry packets on the ground station side! WOOP

# 10-22-2026
Took some time off to focus on other things. But Im back, and now I am deciding how i want to communicate with clients.
Decided that I want to use websockets for communication between server and clients.

# 10-23-2026
Okay figured i am going to use FastSockets library. I want this to be as small as possible because i am running an a Pi 3b. Able to support 1 client but need to manage 
multiple clients.

# 10-24-2026
Added multiple client to the websocket service. Although the websocket service doesnt do anything yet. Need to connect it to a main run-time loop in the ground-station so I can
deal with the different messages coming in.
Okay added tokio because I need to spawn some threads to manage incoming stream / incoming messages / Outgoing streams / outgoing messages.
May have to setup a 2 websocket solution.
Desided also that I need to come up with a log format when I want to load back data. But will have to figure that out once i get the pipeline.

# 10-27-2026
Websocket servers are connected! I decided that there should be a message websocket and a stream socket so that I can parce packets and messages in different threads.
I dont want the stream of packets to interupt messages coming from clients. (when i decide to integrate that). But for now, Lets just have the ground station pass the packets
to the clients.