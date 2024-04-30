use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

