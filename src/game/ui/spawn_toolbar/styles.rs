use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const SPAWN_TOOLBAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute; // Needed to display separately from HUD.
    style.display = Display::Flex; // Hidden by Default
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::FlexEnd;
    style.height = Val::Percent(100.0);
    style.width = Val::Percent(100.0);

    style
};

pub const SPAWN_TOOLBAR_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.height = Val::Percent(10.00);
    style.width = Val::Percent(100.0);

    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.height = Val::Px(80.0);
    style.width = Val::Px(200.0);

    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
