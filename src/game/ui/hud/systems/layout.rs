use bevy::prelude::*;

use crate::game::ui::hud::components::{EnemyText, ScoreText, HUD};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(15.0),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Percent(80.0),
                        margin: UiRect::new(
                            Val::Px(32.0),
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Val::Px(0.0),
                        ),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 0.5)),
                ))
                .with_children(|parent| {
                    // == Star Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/star.png")),
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(48.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                    ));

                    // == Star Counter ==
                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        ScoreText {},
                    ));
                });

            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        margin: UiRect::new(
                            Val::Px(0.0),
                            Val::Px(32.0),
                            Val::Px(0.0),
                            Val::Px(0.0),
                        ),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 0.5)),
                ))
                .with_children(|parent| {
                    // == Enemy Image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(48.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                    ));

                    // == Enemy Counter ==
                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        EnemyText {},
                    ));
                });
        })
        .id()
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
