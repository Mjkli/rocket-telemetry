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

# 10-28-2026
Okay websocket servers are steup and working. After reviewing some logs from the packets i am getting. I am seeing packet loss. So I want to go back and review the packet
parcer.
```
anthony@raspberrypi:~ $ ./ground-station                                                                                                                                                                    
Servers have started: 8080 : 8081                                                                                                                                                                           
Received packet: TelemetryData { roll: 2.5890706264835877, pitch: 0.415722726200279, altitude: 3559.821, temperature: 25.843466002639616, flight_state: Preflight, count: 1 }                               
Received packet: TelemetryData { roll: 2.4477287412570883, pitch: 0.7105103029351144, altitude: 7.836, temperature: 24.29936706402841, flight_state: Preflight, count: 2 }                                  
Received packet: TelemetryData { roll: 2.3497879417597964, pitch: 0.7936213637778068, altitude: 30.161, temperature: 24.394080965014656, flight_state: Preflight, count: 3 }                                
Received packet: TelemetryData { roll: 2.374171122416272, pitch: 0.7247840979972708, altitude: 30.078, temperature: 24.456392799277456, flight_state: Preflight, count: 4 }                                 
Received packet: TelemetryData { roll: 2.3878275904485133, pitch: 0.6937360099664646, altitude: 29.478, temperature: 24.506242299429505, flight_state: Preflight, count: 5 }                                
Received packet: TelemetryData { roll: 2.382220259172809, pitch: 0.705572164990613, altitude: 29.267, temperature: 24.519327798042013, flight_state: Preflight, count: 6 }                                  
Received packet: TelemetryData { roll: 2.4101231636782017, pitch: 0.669134994501549, altitude: 29.363, temperature: 24.543006324438558, flight_state: Preflight, count: 7 }                                 
Received packet: TelemetryData { roll: 2.4408333894066008, pitch: 0.68065819487242, altitude: 29.395, temperature: 24.556714947985256, flight_state: Preflight, count: 8 }                                  
Received packet: TelemetryData { roll: 2.450987346211443, pitch: 0.740087676099621, altitude: 29.182, temperature: 24.56668485740165, flight_state: Preflight, count: 9 }                                   
Received packet: TelemetryData { roll: 2.473454057472706, pitch: 0.6731463058570787, altitude: 28.785, temperature: 24.60220266915858, flight_state: Preflight, count: 18 }                                 
Received packet: TelemetryData { roll: 2.492293039217657, pitch: 0.6940687888500984, altitude: 28.892, temperature: 24.59908707104487, flight_state: Preflight, count: 19 }                                 
Received packet: TelemetryData { roll: 2.462086465828761, pitch: 0.688006385027658, altitude: 28.907, temperature: 24.604695147731398, flight_state: Preflight, count: 20 }                                 
Received packet: TelemetryData { roll: 2.4811840927718194, pitch: 0.6884362154434047, altitude: 28.994, temperature: 24.602825788794963, flight_state: Preflight, count: 21 }                               
Received packet: TelemetryData { roll: 2.5093993862217054, pitch: 0.7148163659779246, altitude: 28.884, temperature: 24.603448908435894, flight_state: Preflight, count: 22 }                               
Received packet: TelemetryData { roll: 2.5274880854794963, pitch: 0.6938916005297796, altitude: 28.958, temperature: 24.603448908435894, flight_state: Preflight, count: 23 }                               
Received packet: TelemetryData { roll: 2.5219609423032843, pitch: 0.6692407097918434, altitude: 29.068, temperature: 24.605941387045096, flight_state: Preflight, count: 24 }                               
Received packet: TelemetryData { roll: 2.527010962907518, pitch: 0.650692116480878, altitude: 28.924, temperature: 24.604072028081372, flight_state: Preflight, count: 25 }                                 
Received packet: TelemetryData { roll: 2.523678641200638, pitch: 0.6701322109352589, altitude: 28.731, temperature: 24.609680105095322, flight_state: Preflight, count: 26 }                                
Received packet: TelemetryData { roll: 2.53725216758553, pitch: 0.666714135098063, altitude: 29.065, temperature: 24.609056985408916, flight_state: Preflight, count: 27 }                                  
Received packet: TelemetryData { roll: 2.5081914948592394, pitch: 0.6195914960874325, altitude: 29.1, temperature: 24.614041943027495, flight_state: Preflight, count: 28 }                                 
Received packet: TelemetryData { roll: 2.5688453057699316, pitch: 0.5881159104294439, altitude: 29.171, temperature: 24.620273140459993, flight_state: Preflight, count: 34 }                               
Received packet: TelemetryData { roll: 2.5499720587628247, pitch: 0.5849769340163822, altitude: 28.976, temperature: 24.617780661432427, flight_state: Preflight, count: 35 }                               
Received packet: TelemetryData { roll: 2.575644384390436, pitch: 0.6066102963469958, altitude: 29.172, temperature: 24.620896260228257, flight_state: Preflight, count: 36 }                                
Received packet: TelemetryData { roll: 2.5913795281613186, pitch: 0.6309857016173369, altitude: 29.215, temperature: 24.622765619560322, flight_state: Preflight, count: 37 }
```


# 10-29-2026
Updated the parcing logic to look for the first 0x00 and the next 0x00. Once we slowed down the fight-computer transmition it made it stable.