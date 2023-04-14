use bevy::prelude::*;

#[derive(Component)]
pub struct FaceCamera; // tag entity to make it always face the camera

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>, // indices of all the frames in the animation
    pub current: usize,
    pub timer: Timer,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState { Loading, MainMenu, Staging, Ready }

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Door;

#[derive(Component)]
struct Name(String);

#[derive(Debug, Clone, Default)]
pub struct CommandCompletedEvent;

#[derive(Debug, Clone, Default)]
pub struct MenuStartEvent;

#[derive(Resource)]
pub struct CommandCompleteIndicator {
    pub completed: bool
}

impl Default for CommandCompleteIndicator {
    fn default() -> Self {
        CommandCompleteIndicator { completed: false }
    }
}