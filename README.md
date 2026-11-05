# bevy-fps-shooter

A small first-person target-practice game written in Rust with [Bevy](https://bevyengine.org/) 0.13 and `bevy_rapier3d` for physics. It was my introduction to game development and Rust: the code is unoptimized and some bugs remain. I'm not working further on this project; it stays as is.

## Requirements
- Rust (stable, 2021 edition) and Cargo
- Bevy's [Linux system dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) on Linux. The `wayland` feature is enabled in `Cargo.toml`.

## Run
```bash
git clone https://github.com/Tjabooo/bevy-fps-shooter.git
cd bevy-fps-shooter
cargo run --release
```
The first build compiles Bevy and takes a while. Run it from the repository root so the `assets/` folder is found.

## Controls
Controls as found in the source:

| Input | Action |
| --- | --- |
| W A S D | Move |
| Mouse | Look |
| Left mouse button | Shoot |
| Space | Jump |
| Left Shift / Left Ctrl | Sprint / crouch |
| E | Interact |
| Escape | Pause menu |

## Project layout
- `src/main.rs`: app setup and state handling.
- `src/modules/`: audio, controls, game logic, gunplay, menus.
- `src/rendering/`: entities and lighting.
- `assets/`: textures, levels (PNG level maps), models, sounds and a font.

## Assets
The map (`de_dust2.glb`), rifle model and sounds are fan-made/derived from Counter-Strike, used here for a non-commercial learning project. They belong to their respective owners and are not covered by this repository's license.
