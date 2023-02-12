use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub struct GameAudioPlugin;

struct AudioState {
    // bgm_handle: Handle<AudioSource>,
    typing_handle: Handle<AudioSource>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(AudioPlugin)
        // .add_startup_system(load_audio);
    }
}

fn load_audio(
    mut commands: Commands,
    audio: Res<Audio>,
    assets: Res<AssetServer>, 
) {
    // let typing_handle = assets.load("audio/assets_sounds_typing.ogg");

//     commands.insert_resource(AudioState{
//         typing_handle: typing_handle
//     });
}