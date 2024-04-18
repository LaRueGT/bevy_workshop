use bevy::prelude::*;
use bevy::text::BreakLineOn;
use crate::ui::ui_components::*;
use crate::ui::ui_styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { let main_menu_entity = build_main_menu(&mut commands, &asset_server); }

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
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