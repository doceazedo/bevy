//! Skinned mesh example with mesh and joints data loaded from a glTF file.
//! Example taken from <https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_019_SimpleSkin.md>

use bevy::prelude::*;
use bevy::pbr::GlobalSkinnedMeshSettings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            brightness: 750.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_settings_and_spawn_mesh)
        .run();
}

fn setup(mut commands: Commands) {
    // Create a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

fn update_settings_and_spawn_mesh(maybe_skinned_mesh_settings: Option<ResMut<GlobalSkinnedMeshSettings>>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if let Some(mut skinned_mesh_settings) = maybe_skinned_mesh_settings {
        if skinned_mesh_settings.max_joints == 512 {
            return;
        };

        skinned_mesh_settings.max_joints = 512;
        skinned_mesh_settings.max_morph_weights = 128;

        commands.spawn(SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/player_character_v2.glb"),
        )));
    }
}
