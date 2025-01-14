use bevy::prelude::*;

use crate::game::ui::game_over_menu::{
    components::{FinalScoreText, GameOverMenu, MainMenuButton, QuitButton, RestartButton},
    styles::NORMAL_BUTTON_COLOR,
};

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ZIndex(2),
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(400.0),
                        height: Val::Px(500.0),
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 0.5)),
                ))
                .with_children(|parent| {
                    // == Title ==
                    parent.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));

                    // == Final Score Text ==
                    parent.spawn((
                        Text::new("Your final score was:"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor::WHITE,
                        FinalScoreText {},
                    ));

                    // == Restart Button ==
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON_COLOR.into()),
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Restart"),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });

                    // == Main Menu Button ==
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON_COLOR.into()),
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Main Menu"),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });

                    // == Quit Button ==
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON_COLOR.into()),
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });
                });
        })
        .id()
}
