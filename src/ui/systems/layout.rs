use bevy::prelude::*;

use crate::configuration::resources::Configuration;
use crate::ui::components::*;
use crate::ui::styles::*;

pub fn build_ui(
    mut commands: Commands,
    config: Res<Configuration>,
    asset_server: Res<AssetServer>,
) {
    let wrapper = NodeBundle {
        style: WRAPPER_STYLE,
        ..default()
    };

    commands.spawn(( wrapper, UI )).with_children(|parent| {
        spawn_configuration(parent, &config, &asset_server);
    });
}

pub fn rebuild_ui(
    mut commands: Commands,
    config: Res<Configuration>,
    asset_server: Res<AssetServer>,
    node_query: Query<Entity, With<UI>>,
) {
    for entity in node_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    build_ui(commands, config, asset_server);
}

fn spawn_configuration(
    parent: &mut ChildBuilder,
    config: &Res<Configuration>,
    asset_server: &Res<AssetServer>,
) {
    let wrapper = NodeBundle {
        style: CONFIGURATION_WRAPPER_STYLE,
        background_color: UI_BACKGROUND_COLOR,
        border_color: UI_BORDER_COLOR,
        border_radius: UI_BORDER_RADIUS,
        ..default()
    };
    let title_wrapper = NodeBundle {
        style: TITLE_WRAPPER_STYLE,
        ..default()
    };
    let title = TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                "Configuration",
                get_text_style(&asset_server, 24.0),
            )],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };
    let content_wrapper = NodeBundle {
        style: CONTENT_WRAPPER_STYLE,
        ..default()
    };

    parent.spawn(wrapper).with_children(|parent| {
        parent.spawn(title_wrapper).with_children(|parent| {
            parent.spawn(title);
        });
        parent.spawn(content_wrapper).with_children(|parent| {
            spawn_checkbox_field(parent, config, asset_server, CheckboxKind::Dust);
            spawn_checkbox_field(parent, config, asset_server, CheckboxKind::Filament);
            spawn_checkbox_field(parent, config, asset_server, CheckboxKind::HII);
            spawn_button_field(parent, config, asset_server, ButtonKind::StarCount);
            spawn_button_field(parent, config, asset_server, ButtonKind::StarSize);
            spawn_button_field(parent, config, asset_server, ButtonKind::Velocity);
            spawn_button_field(parent, config, asset_server, ButtonKind::SemiAxis);
            spawn_button_field(parent, config, asset_server, ButtonKind::OrbitRotation);
            spawn_button_field(parent, config, asset_server, ButtonKind::OrbitDensity);
        });
    });
}

fn spawn_checkbox_field(
    parent: &mut ChildBuilder,
    config: &Res<Configuration>,
    asset_server: &Res<AssetServer>,
    kind: CheckboxKind,
) {
    let text;
    let checked;

    match kind {
        CheckboxKind::Dust => {
            text = "Dust";
            checked = config.dust;
        }
        CheckboxKind::Filament => {
            text = "Filament";
            checked = config.filament;
        }
        CheckboxKind::HII => {
            text = "H-II";
            checked = config.h2;
        }
    }

    let wrapper = NodeBundle {
        style: FIELD_WRAPPER_STYLE,
        ..default()
    };
    let label = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: text.to_string(),
                style: get_text_style(asset_server, 20.0),
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };
    let checkbox = NodeBundle {
        style: CHECKBOX_STYLE,
        border_color: UI_BORDER_COLOR,
        border_radius: UI_BORDER_RADIUS,
        background_color: ELEMENT_BACKGROUND_COLOR,
        ..default()
    };
    let marker = TextBundle {
        visibility: if checked {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        text: Text {
            sections: vec![TextSection {
                value: "X".to_string(),
                style: get_text_style(asset_server, 20.0),
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };

    parent.spawn(wrapper).with_children(|parent| {
        parent.spawn(label);
        parent
            .spawn((checkbox, Checkbox { checked, kind }, Interaction::default()))
            .with_children(|parent| {
                parent.spawn((marker, Marker));
            });
    });
}

fn spawn_button_field(
    parent: &mut ChildBuilder,
    config: &Res<Configuration>,
    asset_server: &Res<AssetServer>,
    kind: ButtonKind,
) {
    let wrapper = NodeBundle {
        style: FIELD_WRAPPER_STYLE,
        ..default()
    };
    let label = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: kind.to_string(),
                style: get_text_style(asset_server, 20.0),
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };
    let value = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: config.map_to_button_value(kind),
                style: get_text_style(asset_server, 14.0),
            }],
            justify: JustifyText::Right,
            ..default()
        },
        style: BUTTON_VALUE_STYLE,
        ..default()
    };

    parent.spawn(wrapper).with_children(|parent| {
        parent.spawn(label);
        parent.spawn(value);
        spawn_button(parent, asset_server, kind);
    });
}

fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, kind: ButtonKind) {
    let button_wrapper = NodeBundle {
        style: BUTTON_WRAPPER_STYLE,
        ..default()
    };
    let increment_button = NodeBundle {
        style: BUTTON_STYLE,
        border_radius: UI_BORDER_RADIUS,
        background_color: ELEMENT_BACKGROUND_COLOR,
        ..default()
    };
    let increment_button_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: String::from("+"),
                style: get_text_style(asset_server, 20.0),
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };
    let decrement_button = NodeBundle {
        style: BUTTON_STYLE,
        border_radius: UI_BORDER_RADIUS,
        background_color: ELEMENT_BACKGROUND_COLOR,
        ..default()
    };
    let decrement_button_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: String::from("-"),
                style: get_text_style(asset_server, 20.0),
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    };

    parent.spawn(button_wrapper).with_children(|parent| {
        parent
            .spawn((
                increment_button,
                TweakButton {
                    action: ButtonAction::Increment,
                    kind,
                },
                Interaction::default(),
            ))
            .with_children(|parent| {
                parent.spawn(increment_button_text);
            });
        parent
            .spawn((
                decrement_button,
                TweakButton {
                    action: ButtonAction::Decrement,
                    kind,
                },
                Interaction::default(),
            ))
            .with_children(|parent| {
                parent.spawn(decrement_button_text);
            });
    });
}
