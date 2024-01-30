use bevy::prelude::*;
use bevy_asm::BevyAsmPlugin;
use bevy_wry::UrlResource;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::BevyWryPlugin {
            as_child: true,
            url: UrlResource("file:///Users/aoamne/Source/bewry/bewry/web/index.html".to_owned()),
            ..default()
        })
        .add_plugins(BevyAsmPlugin {
            use_in_memory_db: true,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
