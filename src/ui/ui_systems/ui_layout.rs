use bevy::prelude::*;
use bevy::text::BreakLineOn;
use crate::ui::ui_components::*;
use crate::ui::ui_styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { let main_menu_entity = build_main_menu(&mut commands, &asset_server); }

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.get_single() {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>) -> Entity
{
    let main_menu_entity = commands.spawn(
        (NodeBundle {
            style: MAIN_MENU_STYLE,
            ..default()
        },
         MainMenu {},
        ))
        .with_children(|parent| {
            //title
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                }).with_children(|parent| {
                //image1
                parent.spawn(
                    ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });
                //text
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Space Busters!",
                            get_title_text_style(&asset_server),
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                        ..default()
                    },
                    ..default()
                });
                //image2
                parent.spawn(
                    ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
            });
            //play button
            parent.spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {},
            ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                            ..default()
                        },
                        ..default()
                    });
                });
            //quit button
            parent.spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {},
            ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    main_menu_entity
}

pub fn build_game_over_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>) -> Entity 
{
    let game_over_menu_entity = commands
        .spawn((
            NodeBundle {
                style: GAME_OVER_MENU_STYLE,
                z_index: ZIndex::Local(2), // See Ref. 1
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(&asset_server),
                            )],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                            ..default()
                        },
                        ..default()
                    });
                    // Final Score Text
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Your final score was:",
                                    get_final_score_text_style(&asset_server),
                                )],
                                justify: JustifyText::Center,
                                linebreak_behavior: BreakLineOn::NoWrap,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));
                    // Restart Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Restart",
                                        get_button_text_style(&asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    linebreak_behavior: BreakLineOn::NoWrap,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_button_text_style(&asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    linebreak_behavior: BreakLineOn::NoWrap,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        get_button_text_style(&asset_server),
                                    )],
                                    justify: JustifyText::Center,
                                    linebreak_behavior: BreakLineOn::NoWrap,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();
    game_over_menu_entity
}