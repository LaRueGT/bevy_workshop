use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Deref, DerefMut)]
pub struct CooldownTimer(pub Timer);
