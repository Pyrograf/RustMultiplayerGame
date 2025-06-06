# Rust Multiplayer Game

One day I will create my own game. Maybe this time!

## Architecture

Concepts: 
- **Account** - username, password, multiple characters
- **Character** - thing user move around in game world
- **Game client** consists of:
  - creating user account
  - logging into user account, retrieving characters list
  - managing characters
  - logging in to world with selected character

Workspace crates idea:
- **accounts_manager** - server (Axum, accounts storage) binary + client library, testable as client-server
- **game_server** - server (TCP, keep characters) binary + client library, testable as client-server
- **game_client** - client (GUI) binary, uses accounts_manager lib, and game_server lib

## What this game should look like?

General concepts:
- Tibia features:
  - graphics: 2D X/Y aligned, fake Z depth,
  - retro graphics
  - tile snapped
  - rare loot
  - tresure chests
  - items tossing on floor
- Runescape:
  - crafting 
  - resources gathering
  - grinding resources
  - resource, production, banking, trading areas
  - fixed inventory, maybe slight variation - bottleneck
- WoW:
  - Money system
  - Resource sourceses renewable with longer intervals
- Added:
  - small combat, maybe only hunting
  - crafting grindig less devastating,
  - quests, dungeons

To consider:
- food needed?
- is dungeon combat based to get some resources?
- dungeon as crafting challenge, example: gather some dungeon specific resources, craft some parts. Shipyard dungeon - gather wood, create wooden planks, repair the deck. Workshop dungeon: gather waste metal, craft cogs, repar machine. Mybe some puzzle to solve to add hardness.

Techs:
- multiplayer - why? Good separation of logic, character || client, graphics
- web cleint via browser - why? Easy access, but not sure if mobile friendly, rather mouse + keybord dedicated.
- macroquad

## Plans

Roadmap:
- [ ] Client entry page with Wasm and basic GUI elements lib