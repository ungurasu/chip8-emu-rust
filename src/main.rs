mod emulib;

use bevy::prelude::*;
use emulib::emulib::*;

fn main() {
    let _emu = Emu::new();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Chip8 Emulator".to_string(),
                width: 640.0,
                height: 320.0,
                ..default()
            },
            ..default()
        }))
        .run();
}