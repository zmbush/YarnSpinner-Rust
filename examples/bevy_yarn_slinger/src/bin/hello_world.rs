use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_dialogue_view::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Register the Yarn Slinger plugin using its default settings, which will look for Yarn files in the "dialogue" folder.
        // If this app should support Wasm or Android, we cannot load files without specifying them, so use the following instead.
        // .add_plugin(YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("dialogue/hello_world.yarn")))
        .add_plugin(YarnSlingerPlugin::new())
        // Initialize the bundled example UI
        .add_plugin(ExampleYarnSlingerDialogueViewPlugin::new())
        .add_systems(
            Update,
            (
                setup_camera.on_startup(),
                // Spawn the dialogue runner once the Yarn project has finished compiling
                spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner();
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
