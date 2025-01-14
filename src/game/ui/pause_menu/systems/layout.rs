use bevy::prelude::*;

use crate::game::ui::pause_menu::components::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
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
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(400.0),
                        height: Val::Px(400.0),
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 0.5)),
                ))
                .with_children(|parent| {
                    // == Title ==
                    parent.spawn((
                        Text::new("Pause Menu"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // == Resume Button ==
                    parent
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            ResumeButton {},
                        ))
                        .with_child((
                            Text::new("Resume"),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    // == Main Menu Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            MainMenuButton {},
                        ))
                        .with_child((
                            Text::new("Main Menu"),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    // == Quit Button ==
                    parent
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                column_gap: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            QuitButton {},
                        ))
                        .with_child((
                            Text::new("Quit"),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                });
        })
        .id();

    pause_menu_entity
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}
