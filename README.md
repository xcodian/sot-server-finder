# SoT Server Finder

Find which Sea of Thieves server you're connected to. Useful if you want to be in the same server as your friends. 

```
Looking for Sea of Thieves...
Found! PID: 21148
Waiting for you to connect to a game in Sea of Thieves...
You are connected to: berlin-stream-montana-johnny (20.213.146.107:30401)
```

## Setup
1. [Download sot-server-finder.exe from GitHub Releases](https://github.com/xxcodianxx/sot-server-finder/releases/download/0.1.0/sot-server-finder.exe) or build it yourself.
2. Download and run [the Npcap installer](https://npcap.com/dist/npcap-1.72.exe). Select WinPCap compatibility mode. 
3. Follow usage steps below to use it!

## Usage
Start the program, and if you haven't already, start Sea of Thieves.

On the occasion that your computer is not connected to a `192.168.x.x` subnet, the program will prompt you to select your outbound adapter. 
Most home users won't have to worry about this. If you're not getting prompted, don't worry about it.

The program will tell you what server you're connected to in a friendly format so you can quickly cross reference with your friends.


## Building
This works only on Windows.

1. Download and run [the Npcap installer](https://npcap.com/dist/npcap-1.72.exe). Select WinPCap compatibility mode. 
2. Run `cargo build --release`.

The [build script](build.rs) will automatically download [the Npcap SDK version 1.13](https://npcap.com/dist/npcap-sdk-1.13.zip) and place it in the `libs` directory.

Your exe file will be `target/release/sot-server-finder.exe`.

## Also See
- Someone did this 3 years ago in C#: https://github.com/Saeryhz/SeaOfEase