# mince-raft-rs

A Minecraft client to connect to vanilla Minecraft servers written in Rust.

I can only do this thanks to the extremely detailed documentation on the Minecraft Protocol at https://wiki.vg/Protocol

This is just a personal project to learn and practice, don't expect it to become useable as a replacement for the real Minecraft client.

![Minecraft Client](journal/Client.png)


# Compiling/Running

Currently this does not have any title menu or settings and just automatically connects to a server to play. If you want to try the current client, you will need to start a Minecraft server (like Spigot) and turn off authentication, set the IP of the server in `src/client.rs` to your server and recompile the project.
(You will probably also need to enable flying as well to prevent the client from getting kicked regularly)

# Current Features

The client currently only has some basic functionality:
* Connect to unsecured vanilla Minecraft servers

* Send a receive basic chat messages with the server and execute commands

![Chat Being Used](journal/Chat.gif)

* Understand information about the server and the player

![Information panel](journal/Information.png)

* Get nearby entities and their data

![Entity Information](journal/Entities.png)

* Read Chunk Data and get blocks to construct chunks

![Block Palette for a chunk](journal/Palette.png)

* Render a basic view of the chunks loaded

![Rendering](journal/Rendering.png)

* Fly around in the world

![Flying around in Minecraft](journal/MovementRendering.gif)
