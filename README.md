<img src="https://github.com/user-attachments/assets/1fca61fe-5fc7-4b90-97af-db6e7310259a" width="100%"/>

# shp_vis_bevy

## Installation
### Option1: Release Binaries
Download from [Releases](https://github.com/HellOwhatAs/shp_vis_bevy/releases).
### Option2: Build from Source
Install [Rust](https://www.rust-lang.org/).
```
git clone https://github.com/HellOwhatAs/shp_vis_bevy.git
cd shp_vis_bevy
cargo build --release
```

## Usage
```
shp_vis_bevy path/to/shapefile pos_x pos_y
```
- `path/to/shapefile`: string
- `pos_x`, `pos_y`: float  
  Initial position of the camera.

## Controls
```
Freecam Controls:
    Mouse       - Move camera orientation
    Scroll      - Adjust movement speed
    Left        - Hold to grab cursor
    KeyM        - Toggle cursor grab
    KeyW & KeyS - Fly forward & backwards
    KeyA & KeyD - Fly sideways left & right
    KeyE & KeyQ - Fly up & down
    ShiftLeft   - Fly faster while held
```
