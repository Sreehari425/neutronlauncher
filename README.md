# Neutron Launcher

> **⚠️ WIP (Work In Progress)** - This project is currently under active development. Features may be incomplete or subject to change.

A simple, fast, and lightweight CLI tool to manage and launch your Linux native and Wine games

## Usage

### Create Instance

Create a new game instance with the specified configuration:

```bash
neutron-launcher create-instance --name <NAME> --executable-path <PATH> --type <TYPE> [--description <DESC>]
```

**Arguments:**

- `--name <NAME>`: The display name of the game
- `--executable-path <PATH>`: Path to the game executable
- `--type <TYPE>`: Instance type (`linux-native` or `wine`)
- `--description <DESC>`: Optional description for the game

**Examples:**

```bash
# Create a wine game
neutron-launcher create-instance \
  --name "Cyberpunk 2077" \
  --executable-path "/home/user/games/cyberpunk2077/bin/Cyberpunk2077.exe" \
  --type wine \
  --description "Futuristic RPG game"

# Create linux native game
neutron-launcher create-instance \
  --name "Minecraft Java Edition" \
  --executable-path "/home/user/.minecraft/launcher.jar" \
  --type linux-native
```

### List Instances

Display all saved game instances:

```bash
neutron-launcher list-instances
```

**Output example:**

```
[LAUNCHER] Launcher directory exists at /home/user/.config/NeutronLauncher
Found 2 instance(s):
  ID: cyberpunk_2077
    Name: Cyberpunk 2077
    Path: /home/user/games/cyberpunk2077/bin/Cyberpunk2077.exe

  ID: minecraft_java_edition
    Name: Minecraft Java Edition
    Path: /home/user/.minecraft/launcher.jar
```

### Remove Instance

Remove a game instance by its ID:

```bash
neutron-launcher remove-instance --id <INSTANCE_ID>
```

**Arguments:**

- `--id <INSTANCE_ID>`: The ID of the instance to remove (shown in `list-instances`)

**Example:**

```bash
neutron-launcher remove-instance --id "minecraft_java_edition"
```

### Help

Get help for any command:

```bash
# General help
neutron-launcher --help

# Help for specific command
neutron-launcher create-instance --help
neutron-launcher list-instances --help
neutron-launcher remove-instance --help
```

## Configuration

Game instances are stored in `~/.config/NeutronLauncher/config.json` with the following structure:

```json
{
  "instances": {
    "game_id": {
      "game_name": "Game Display Name",
      "game_path": "/path/to/game/executable"
    }
  }
}
```

## TODO

- [x] Implement `create-instance` command:
  - [x] Support for `wine` and `linux-native` game types
  - [x] Store game name, executable path, and optional description
- [x] Implement `list-instances` command to show all saved game instances
- [x] Implement `remove-instance` command to remove a game instance by ID
- [x] Store game instances persistently in JSON file
- [x] Add helpful CLI messages and error handling with colored output
- [ ] Implement game launching functionality
- [ ] Add configuration validation (check if executable exists)
- [ ] Add support for game arguments/launch options
- [ ] Implement game categories/tags
- [ ] Add import/export functionality for game configurations
