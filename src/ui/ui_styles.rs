use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.25, 0.25); 
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let mut style: TextStyle = TextStyle::default();
    style.font = asset_server.load("fonts/FiraSans-Bold.ttf");
    style.font_size = 32.0;
    style.color = Color::rgb(1.0, 1.0, 1.0);
    style
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let mut style: TextStyle = TextStyle::default();
    style.font = asset_server.load("fonts/FiraSans-Bold.ttf");
    style.font_size = 64.0;
    style.color = Color::rgb(1.0, 1.0, 1.0);
    style
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let mut style: TextStyle = TextStyle::default();
    style.font = asset_server.load("fonts/FiraSans-Bold.ttf");
    style.font_size = 48.0;
    style.color = Color::rgb(1.0, 1.0, 1.0);
    style
}

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(300.0);
    style.height = Val::Px(120.0);
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const GAME_OVER_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.display = Display::Flex;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(400.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};