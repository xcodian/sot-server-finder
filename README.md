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
Start the program as Administrator, and if you haven't already, start Sea of Thieves.

On the occasion that your computer is not connected to a `192.168.x.x` subnet, the program will prompt you to select your outbound adapter. 
Most home users won't have to worry about this. If you're not getting prompted, don't worry about it.

The program will ask you what server you want to connect to.
- Here you should get one of your friends to run this. They should type "idk" and set sail in Sea of Thieves. The program will show the server that they connected to.

Now you can enter the IP that your friend got, and begin the grind.

**The Grind**

With this software, you don't have to fully load in to the tavern in order to hop to a new server. This program will drop the connection between Sea of Thieves and its server when it detects that you connected to a server you didn't want. Despite this, it takes some work from you to keep setting sail.

1. The program should be waiting for a connection.
2. Set sail as you would, make sure you have enough space for all friends, etc.
3. The program will automatically detect a connection and see if it's to the server you want. If it isn't, it will drop the connection.
4. After a few seconds, the game will kick you back out saying "failed to connect". Go through the prompts, but very importantly, **answer no when the game asks you if you'd like to reconnect to your previous session!**
5. After this, hit Enter in the program window. It should unblock your connection and wait for another one.
6. Rinse and repeat until you hit the server you want.

The more friends you get to do this grind the merrier! Happy hopping!


## Building
This works only on Windows.

1. Download and run [the Npcap installer](https://npcap.com/dist/npcap-1.72.exe). Select WinPCap compatibility mode. 
2. Run `cargo build --release`.

The [build script](build.rs) will automatically download [the Npcap SDK version 1.13](https://npcap.com/dist/npcap-sdk-1.13.zip) and place it in the `libs` directory.

Your exe file will be `target/release/sot-server-finder.exe`.

## Also See
- Someone did this 3 years ago in C#: https://github.com/Saeryhz/SeaOfEase