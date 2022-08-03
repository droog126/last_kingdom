#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Playing,
    Menu,
}

pub mod loading;
pub mod origin;
pub mod playing;
