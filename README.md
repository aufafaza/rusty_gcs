# Mavlink Ground Control Station 
An attempt to create a web-based ground control station using rust and MQTT 

# Deps 
- mavlink crate, that's it, so far at least :|

# Roadmap
[this section will be a list of things i did including my design though process] 
## First Day
1. First thing to do is connecting to an UDP port (should be testable with ardupilot SITL or an FC),then synchronously parse messages, e.g. attitude and altitude. 
2. Then we would want to change step 1 to an asynchronous I/O without thread blocking (i.e. using tokio::spawn, instead of await).

Now async connection is pretty much done, the next day will be an attempt on creating a websocket and MQTT. Will only still be reading the message stream. 

## Second day 
1. Use MQTT broker to create an event loop for publishing a message, to try, use muquitto
