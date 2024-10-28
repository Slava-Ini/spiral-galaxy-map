use bevy::prelude::*;

use crate::configuration::resources::*;
use crate::ui::components::*;
use crate::ui::styles::*;

pub fn interact_with_checkbox(
    mut interaction_query: Query<
        (
            &mut Checkbox,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Visibility, With<Marker>>,
    mut config: ResMut<Configuration>,
) {
    for (mut checkbox, interaction, mut background_color, mut border_color, children) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                checkbox.checked = !checkbox.checked;

                match checkbox.kind {
                    CheckboxKind::Dust => config.dust = checkbox.checked,
                    CheckboxKind::HII => config.h2 = checkbox.checked,
                    CheckboxKind::Filament => config.filament = checkbox.checked,
                }
            }
            _ => {}
        }

        if checkbox.checked {
            *background_color = CHECKBOX_BACKGROUND_COLOR_CHECKED.into();
            *border_color = CHECKBOX_BORDER_COLOR_CHECKED.into();
        } else {
            *background_color = ELEMENT_BACKGROUND_COLOR.into();
            *border_color = UI_BORDER_COLOR.into();
        }

        for &child in children.iter() {
            if let Ok(mut visibility) = text_query.get_mut(child) {
                *visibility = if checkbox.checked {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
        }
    }
}

pub fn interact_with_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &TweakButton),
        Changed<Interaction>,
    >,
    mut config: ResMut<Configuration>,
) {
    for (interaction, mut background_color, &button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = ELEMENT_BACKGROUND_COLOR_PRESSED.into();
                config.update(button.kind, button.action);
            }
            Interaction::Hovered => {
                *background_color = ELEMENT_BACKGROUND_COLOR_HOVERED.into();
            }
            Interaction::None => {
                *background_color = ELEMENT_BACKGROUND_COLOR.into();
            }
        }
    }
}

