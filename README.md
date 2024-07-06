# Lhue
> (via version 0.1.0)  

This is just a project to practice Rust and solve a problem of mine.

# Features

`lhue room on`  - _turns on all of the rooms lights_  
`lhue room off` - _turns off all of the rooms lights_

# How does it work?
1. You have to manually get your apikey from Philips Hue via registering the current device, that you are using. _(this will be avaliable later in the cli)_
2. Create .cargo directory and a config.toml file inside
```
touch .cargo/config.toml
```
3. Set the environment variables
```
[env]
HUE_BRIDGE_LOCAL_IP = ""
HUE_APP_KEY = ""
ROOM_ID = ""
```
4. Build 
```
cargo build --release
```
5. Install
```
cargo install --path .
```  

# Planned features
### General
- Login functionality, so you don't have to open the Philips Hue Site for registering your device
- Raycast extension
### Functionality
- Selecting different rooms and devices
- Dimming Lights
- Changing colors