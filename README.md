# Discord VN Engine
A visual novel scripting language which renders as interactable components on discord. To get started on scripting, visit [our friendly guide](https://github.com/grostaco/discord_vn/blob/main/docs/README.md). If you'd like to contact me, my discord is `(trait*)sbrk(sizeof *traits)#1732`.

# Table of contents
- [Usage](#usage)
    - [Installation](#installation)
        - [Windows](#windows)
        - [Linux](#linux-and-macos)
        - [MacOS](#linux-and-macos)
- [Scripting](#scripting)
    - [Dialogue](#dialogue)
    - [Directives](#meta-directives)
    - [Examples](#examples)
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
For testing scripts, it is recommended to use the engine environment available for download [here](https://github.com/grostaco/discord_vn/releases/latest/download/engine.zip).
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

Meta direcitves are powerful commands to control the environment of the script. They can be invoked by `@directive(arguments)`.

| Directive | Arguments           | Notes                         |
|-----------|---------------------|-------|
| loadbg    | path                | load background image at path |
| jump      | path                | Unconditionally jumps to the script file at path|
| jump      | text,text,path      | Render two text choices and jump to script file at `path` if the first argument is chosen|
| sprite | text,path,int,int,show | Create a sprite with the first argument's name from path with the third and forth argument horizontal and vertical placement of the sprite respectively.
| sprite | text,hide | Hide the sprite with the first argument's name |
| custom | directive(args) | Pass a custom directive to any frontend programs using the engine. Ignored by the engine. This serves as a complementary comment
| custom(play) | snowflake, snowflake, text | If ran on a discord engine, play a song from the third argument's url in the first argument's guild and second argument's voice channel. This is an unstable feature and highly subject to change in upcoming versions.
The screen size is set at exactly 640x480, images and sprite positioning should match this.

## Examples

```ini
@loadbg(resources/bgs/living_room.png)
[John]
John is saying something in the living room
```

```ini
@loadbg(resources/bgs/living_room.png)
@sprite(john,resources/sprites/john.png,320,240,show)
[John]
John is saying something in the living room with him being at the exact center
```
---
```ini
# resources/scripts/script.txt

@loadbg(resources/bgs/living_room.png)
@sprite(john,resources/sprites/john.png,320,240,show)
[John]
John is saying something in the living room with him being at the exact center
[John]
I am going to head to the next room
[!meta sprite(john,hide)]
[John]
I am already out of the room, follow me!
@jump(resources/scripts/script2.txt)
```
```ini
# resources/scripts/script2.txt
@loadbg(resources/bgs/guest_room.png)
[John]
Welcome to the guest room
[John]
Would you like to leave?
@jump(Yes,No,resources/scripts/end_leave.txt)
@jump(resources/scripts/end_stayed.txt)
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
