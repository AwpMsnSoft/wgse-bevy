use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Component, PartialEq, Eq, Hash)]
pub struct WidgetId(pub i32);

impl Default for WidgetId {
    fn default() -> Self {
        WidgetId(0)
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize)]
pub struct WidgetBundle<SubBundle>
where
    SubBundle: Send + Sync + 'static + Bundle,
{
    pub id: WidgetId,
    #[bundle]
    pub children: SubBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Descriptor {
    pub id: i32,
    #[serde(flatten)]
    pub content: WidgetDescriptor,
    #[serde(default)]
    pub children: Option<GroupDescriptor>,
}

#[macro_export]
macro_rules! descriptor {
    ($id: expr, $content: expr, $children: expr) => {{
        Descriptor {
            id: $id,
            content: $content,
            children: $children,
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
#[allow(non_camel_case_types)]
pub enum WidgetDescriptor {
    button(ButtonDescriptor),
    image(ImageDescriptor),
}

#[allow(unreachable_patterns)]
// TODO: use a macro to generate this
pub fn widget_descriptor_spawn(parent: &mut ChildBuilder, descriptor: &Descriptor) {
    match &descriptor.content {
        WidgetDescriptor::button(button) => {
            debug!("Spawning button: {:?}", button);
            parent.spawn_bundle(WidgetBundle {
                id: WidgetId(descriptor.id),
                children: ButtonBundle::from(button.clone()),
            });
        }
        WidgetDescriptor::image(image) => {
            debug!("Spawning image: {:?}", image);
            parent.spawn_bundle(WidgetBundle {
                id: WidgetId(descriptor.id),
                children: ImageBundle::from(image.clone()),
            });
        }
        _ => panic!("Current WidgetDescriptor is not supported yet."),
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct ButtonDescriptor {
    pub size: Vec2,
    pub position: Vec2,
}

impl From<ButtonDescriptor> for ButtonBundle {
    fn from(descriptor: ButtonDescriptor) -> Self {
        ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Px(descriptor.size.x),
                    height: Val::Px(descriptor.size.y),
                },
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(descriptor.position.x),
                    right: Val::Px(descriptor.position.x + descriptor.size.x),
                    top: Val::Px(descriptor.position.y),
                    bottom: Val::Px(descriptor.position.y - descriptor.size.y),
                },
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! button {
    ($size_x: expr, $size_y: expr, $position_x: expr, $position_y: expr) => {{
        ButtonDescriptor {
            size: Vec2::new($size_x, $size_y),
            position: Vec2::new($position_x, $position_y),
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Scale2D(pub Vec2);

impl Default for Scale2D {
    fn default() -> Self {
        Scale2D(Vec2::new(1.0, 1.0))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct ImageDescriptor {
    pub size: Vec2,
    pub position: Vec2,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub scale: Scale2D,
}

impl From<ImageDescriptor> for ImageBundle {
    fn from(descriptor: ImageDescriptor) -> Self {
        Self {
            style: Style {
                size: Size {
                    width: Val::Px(descriptor.size.x),
                    height: Val::Px(descriptor.size.y),
                },
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(descriptor.position.x),
                    right: Val::Px(descriptor.position.x + descriptor.size.x),
                    top: Val::Px(descriptor.position.y),
                    bottom: Val::Px(descriptor.position.y - descriptor.size.y),
                },
                ..Default::default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(descriptor.rotation),
                scale: Vec3::new(descriptor.scale.0.x, descriptor.scale.0.y, 1.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! image {
    ($size_x: expr, $size_y: expr, $position_x: expr, $position_y: expr) => {{
        ImageDescriptor {
            size: Vec2::new($size_x, $size_y),
            position: Vec2::new($position_x, $position_y),
            ..Default::default()
        }
    }};
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Component)]
pub struct GroupDescriptor(pub Vec<Descriptor>);

#[test]
fn test_image_serialization() {
    let image = Descriptor {
        id: 0,
        content: WidgetDescriptor::image(image!(100.0, 100.0, 0.0, 0.0)),
        children: None,
    };
    let serialized = serde_json::to_string(&image).unwrap();
    println!("{}", serialized);
}

#[test]
fn test_image_deserialization() {
    let serialized = r#"{"id":0,"image":{"size":[100.0,100.0],"position":[0.0,0.0]}}"#;
    let image = serde_json::from_str::<Descriptor>(serialized).unwrap();
    println!("{:?}", image);
}
