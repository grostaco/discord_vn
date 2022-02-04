# Discord VN Engine
A visual novel scripting language which renders as interactable components on discord. 
# Usage

## Installation
### Windows
- Download the latest release from [here](https://github.com/grostaco/discord_vn/releases/latest/download/discord.zip)
- Go to [Discord's developer applications](https://discord.com/developers/applications)
    - Create a new application
    - Click `add bot` in the bot section
    - Go to OAuth2 URL Generator tab at the right
        - Tick `applications.command` and `bot`
        - Select bot permissions `Send Messages`, `Attach files`
        - Use the invite link below to invite the bot your guild
- Edit `resources/config.conf`
    - Copy the bot's token to `discord_token`
    - Copy the bot's application id to `application_id`
    - Make a channel in the guild for images and copy the id to `image_channel`
    - Copy the guild id to `guild_id`
    - Set the game's name to `name`
- Launch `run_discord.bat`
- Use `/begin` to run the bot
### Linux and MacOS
For linux and mac users, install the rust toolchain and pull the repository
```shell
cargo build --release --bin discord
cargo run --bin discord
```

# Scripting
For testing scripts, it is recommended to use the engine environment available for download here [here](https://github.com/grostaco/discord_vn/releases/latest/download/discord.zip).
## Dialogue

To create a dialogue, the format goes as follow

```ini
[Character]
This is character saying something
This is still part of the character dialogue box

[Character]
This is a new dialogue box
```

## Meta directives

Meta direcitves are powerful commands to control the environment of the script. They can be invoked by `[!meta directive(arguments)]`.

| Directive | arguments           | notes                         |
|-----------|---------------------|-------|
| loadbg    | path                | load background image at path |
| jump      | path                | Unconditionally jumps to the script file at path|
| jump      | text,text,path      | Render two text choices and jump to script file at `path` if the first argument is chosen|
| sprite | text,path,int,int,show | Create a sprite with the first argument's name from path with the third and forth argument horizontal and vertical placement of the sprite respectively.
| sprite | text,hide | Hide the sprite with the first argument's name

The screen size is set at exactly 640x480, images and sprite positioning should match this.

Examples:

```ini
[!meta loadbg(resources/bgs/living_room.png)]
[John]
John is saying something in the living room
```

```ini
[!meta loadbg(resources/bgs/living_room.png)]
[!meta sprite(john,resources/sprites/john.png,320,240,show)]
[John]
John is saying something in the living room with him being at the exact center
```
---
```ini
# resources/scripts/script.txt

[!meta loadbg(resources/bgs/living_room.png)]
[!meta sprite(john,resources/sprites/john.png,320,240,show)]
[John]
John is saying something in the living room with him being at the exact center
[John]
I am going to head to the next room
[!meta sprite(john,hide)]
[John]
I am already out of the room, follow me!
[!meta jump(resources/scripts/script2.txt)]
```
```ini
# resources/scripts/script2.txt
[!meta loadbg(resources/bgs/guest_room.png)]
[John]
Welcome to the guest room
[John]
Would you like to leave?
[!meta jump(Yes,No,resources/scripts/end_leave.txt)]
[!meta jump(resources/scripts/end_stayed.txt)]
```
```ini
# resources/scripts/end_leave.txt
[]
You left
```
```ini
# resources/scripts/end_stayed.txt
[]
You stayed
```
