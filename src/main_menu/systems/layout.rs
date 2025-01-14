use bevy::prelude::*;

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::{BUTTON_STYLE, NORMAL_BUTTON_COLOR},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // == Title ==
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(650.0),
                    height: Val::Px(120.0),
                    ..default()
                },))
                .with_children(|parent| {
                    // == Image 1 ==
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                    ));

                    // == Text ==
                    parent.spawn((
                        Text::new("Bevy Ball Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 64.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // == Image 2 ==
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                        Node {
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                                Val::Px(8.0),
                            ),
                            ..default()
                        },
                    ));
                });

            // == Play Button ==
            parent
                .spawn((
                    Button,
                    BUTTON_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_child((
                    Text::new("Play"),
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
                    BUTTON_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
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
        })
        .id();

    main_menu_entity
}
