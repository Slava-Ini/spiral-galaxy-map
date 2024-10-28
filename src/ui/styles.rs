use bevy::prelude::*;

// -- Const Variables
pub const UI_BACKGROUND_COLOR: BackgroundColor = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5));
pub const UI_BORDER_COLOR: BorderColor = BorderColor(Color::WHITE);
pub const UI_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(5.0));
pub const CHECKBOX_BACKGROUND_COLOR_CHECKED: BackgroundColor = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5));
pub const CHECKBOX_BORDER_COLOR_CHECKED: BorderColor = BorderColor(Color::WHITE);
pub const ELEMENT_BACKGROUND_COLOR: BackgroundColor = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5));
pub const ELEMENT_BACKGROUND_COLOR_HOVERED: BackgroundColor = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.5));
pub const ELEMENT_BACKGROUND_COLOR_PRESSED: BackgroundColor = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.5));

// -- Styles
pub const WRAPPER_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.padding = UiRect {
        left: Val::Percent(2.0),
        ..UiRect::all(Val::Auto)
    };

    style
};
pub const CONFIGURATION_WRAPPER_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::FlexStart;
    style.width = Val::Percent(20.0);
    style.height = Val::Percent(70.0);

    style
};
pub const TITLE_WRAPPER_STYLE : Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Percent(100.0);
    style.height = Val::Percent(10.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;

    style
};
pub const CONTENT_WRAPPER_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Percent(90.0);
    style.height = Val::Percent(90.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_items = AlignItems::FlexStart;
    style.padding = UiRect {
        left: Val::Percent(10.0),
        ..UiRect::all(Val::Auto)
    };

    style
};
pub const FIELD_WRAPPER_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Percent(100.0);
    style.height = Val::Px(10.0);
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;

    style
};
pub const CHECKBOX_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(24.0);
    style.height = Val::Px(24.0);

    style
};
pub const BUTTON_WRAPPER_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.width = Val::Px(24.0);
    style.height = Val::Px(24.0);
    style.flex_direction = FlexDirection::Column;

    style
};
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(50.0);

    style
};
pub const BUTTON_VALUE_STYLE: Style = {
    let mut style = Style::DEFAULT;

    style.flex_grow = 1.0;
    style.padding = UiRect {
        right: Val::Px(10.0),
        ..UiRect::all(Val::Auto)
    };

    style
};

// -- Helpers
pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::WHITE,
        ..default()
    }
}
