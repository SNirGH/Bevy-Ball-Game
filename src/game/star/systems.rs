use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::{components::Star, resources::StarSpawnTimer, NUMBER_OF_STARS};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform {
                translation: Vec3::new(random_x, random_y, 0.0),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, stars_query: Query<Entity, With<Star>>) {
    for star_entity in stars_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.just_finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform {
                translation: Vec3::new(random_x, random_y, 0.0),
                ..default()
            },
            Star {},
        ));
    }
}
