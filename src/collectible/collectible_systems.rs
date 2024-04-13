//std
use rand::Rng;
//extern
use bevy::prelude::*;
use bevy::utils::Duration;
//children
use crate::collectible::collectible_components::*;
use crate::collectible::collectible_resources::*;
//parents
use crate::BallCollider;

pub fn spawn_collectibles(
    mut commands: Commands,
    window: Query<&Window>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<CollectibleSpawnTimer>,
    limit: Res<CollectibleSpawnLimit>,
    mut count: ResMut<CollectibleSpawnCount>,
) {
    let win = window.get_single().unwrap();
    let mut rng = rand::thread_rng();
    let half_width = win.resolution.width() / 2.0;
    let half_height = win.resolution.height() / 2.0;

    timer.tick(time.delta());

    if timer.finished() && count.value < limit.value {
        let random_x = rng.gen_range(-half_width..half_width);
        let random_y = rng.gen_range(-half_height..half_height);
        //spawn a collectible
        commands.spawn((
            Collectible,
            SpriteBundle {
                texture: asset_server.load("sprites/star.png"),
                transform: Transform::from_translation(Vec3::new(
                    random_x, random_y, 0.0))
                    .with_scale(Vec3::splat(2.0)),
                ..default()
            },
            BallCollider(10.0),
        ));
        count.value += 1;
        timer.set_duration(Duration::from_millis(rng.gen_range(500..3000)));
        timer.reset();
    }
}
