# 09-23-2026
Okay, Got my idea of how I want the "protocol" to look like. I dont really think it is a protocol since we are just jumping off the 
backbone of Uart / Serial communication. So I think its just more like an encoding. 
It will be a simple library that encodes a given struct to binary. But we also want to add more validation to the "packet".
A Crc block. This will validate that the data didnt get corrupted in transit. 
And a Cobs encoding. This will help parsing the packet when it is received. Actually just found out about COBs today and its interesting.
To find the end of the packet from a continuous stream, you set a "header" block that defines where the 0x00s are in your encoding. This effectively turns into pointers telling where the 0x00s were. Then you can reconstruct the packet after the fact when you retrive it. 
Pretty smart. 
Hooked up this method to the flight computer TelemtryData struct currently.
I think I would like to add an e2e validation also. So I can track if I had a packet get lost. 
