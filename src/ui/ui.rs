use crate::{
    button, descriptor, image,
    ui::{
        descriptors::{
            ButtonDescriptor, Descriptor, GroupDescriptor, ImageDescriptor, WidgetDescriptor,
        },
        resources::*,
    },
};
use bevy::prelude::*;
use lazy_static::lazy_static;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

lazy_static! {
    pub static ref MAIN_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        MAIN_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0, WINDOW_HEIGHT)),
        Some(GroupDescriptor(vec![
            descriptor!(
                MAIN_TITLE_START_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 200.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                MAIN_TITLE_CONFIG_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 300.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                MAIN_TITLE_EXTRA_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 400.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                MAIN_TITLE_EXIT_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 500.0, WINDOW_HEIGHT - 480.0)),
                None
            )
        ]))
    ),];
}

lazy_static! {
    pub static ref EXTRA_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        EXTRA_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0, WINDOW_HEIGHT)),
        Some(GroupDescriptor(vec![
            descriptor!(
                EXTRA_TITLE_CG_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 200.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_SCENE_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 300.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_MUSIC_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 400.0, WINDOW_HEIGHT - 480.0)),
                None
            ),
            descriptor!(
                EXTRA_TITLE_BACK_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(100.0, 40.0, 500.0, WINDOW_HEIGHT - 480.0)),
                None
            )
        ]))
    ),];
}

lazy_static! {
    pub static ref CONFIG_SPEED_TITLE_LAYOUT: Vec<Descriptor> = vec![descriptor!(
        CONFIG_SPEED_TITLE_BG_GUID.0,
        WidgetDescriptor::image(image!(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0, WINDOW_HEIGHT)),
        Some(GroupDescriptor(vec![
            descriptor!(
                CONFIG_SPEED_TITLE_SPEED_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(130.0, 39.0, 62.0, WINDOW_HEIGHT - 514.0)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SOUND_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(130.0, 39.0, 212.0, WINDOW_HEIGHT - 514.0)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_EXTRA_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(130.0, 39.0, 362.0, WINDOW_HEIGHT - 514.0)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_BACK_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(115.0, 48.0, 665.0, WINDOW_HEIGHT - 528.0)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SKIP_READED_ON_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(36.0, 19.0, 372.0, WINDOW_HEIGHT - 344.0)),
                None
            ),
            descriptor!(
                CONFIG_SPEED_TITLE_SKIP_READED_OFF_BUTTON_GUID.0,
                WidgetDescriptor::button(button!(36.0, 24.0, 437.0, WINDOW_HEIGHT - 339.0)),
                None
            )
        ]))
    )];
}
