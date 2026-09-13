use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Ammo, Burst, Cooldown, EquippedBy, Item, ItemFootprint, ItemKey, ItemLabel, ItemViewDefinition,
    ItemViewRegistry, OnGround, Shootable, StoredIn, build_chrome_patch,
};

pub struct RiflePlugin;

impl Plugin for RiflePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Rifle>()
            .add_systems(Startup, register_rifle_view);
    }
}

pub const RIFLE_KEY: &str = "core::item::rifle";

#[derive(SceneComponent, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Rifle;

impl Rifle {
    pub fn scene() -> impl Scene {
        bsn! {
            Item {
                key: {ItemKey(RIFLE_KEY.to_string())},
                label: {ItemLabel("Assault Rifle".to_string())},
                footprint: {ItemFootprint(UVec2 { x: 8, y: 4 })} // BFG of the long variety
            }
            Shootable { cooldown: {Cooldown(1.0)}, magazine_size: 30 }
            Burst { shots: 3, interval: {Cooldown(0.1)} }
            Ammo(30)
            Visibility
        }
    }
}

const COLOR: Color = Color::srgb(0.32, 0.35, 0.34);
const ICON_COLOR: Color = Color::WHITE;
const ICON_BORDER_COLOR: Color = Color::BLACK;
const ICON_BORDER_PX: f32 = 2.0;

fn register_rifle_view(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut views: ResMut<ItemViewRegistry>,
) {
    // Grounded and hand meshes are different to emphasize how
    // the `Rusty` component can be applied in any state of the mesh
    let ground_mesh = meshes.add(Cuboid::new(0.08, 0.18, 1.1));
    let hand_mesh = meshes.add(Sphere::new(0.1));

    let material = materials.add(StandardMaterial::from(COLOR));

    let ground_material = material.clone();
    let ground = build_chrome_patch(
        &asset_server,
        bsn! {
            Mesh3d(ground_mesh)
            MeshMaterial3d<StandardMaterial>(ground_material)
        },
    );

    let equipped = build_chrome_patch(
        &asset_server,
        bsn! {
            Mesh3d(hand_mesh)
            MeshMaterial3d<StandardMaterial>(material)
        },
    );

    let icon_color = ICON_COLOR;
    let icon_border = UiRect::all(Val::Px(ICON_BORDER_PX));
    let icon_border_color = ICON_BORDER_COLOR;

    let stored = build_chrome_patch(
        &asset_server,
        bsn! {
            Node { border: icon_border }
            BackgroundColor(icon_color)
            BorderColor {
                top: icon_border_color,
                right: icon_border_color,
                bottom: icon_border_color,
                left: icon_border_color,
            }
        },
    );

    views.register(
        ItemKey(RIFLE_KEY.to_string()),
        ItemViewDefinition {
            chrome: HashMap::from([
                (OnGround::KEY.to_string(), ground),
                (EquippedBy::KEY.to_string(), equipped),
                (StoredIn::KEY.to_string(), stored),
            ]),
        },
    );
}
