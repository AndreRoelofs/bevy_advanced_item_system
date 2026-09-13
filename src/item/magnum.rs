use bevy::{platform::collections::HashMap, prelude::*, scene::ScenePatch};

use crate::{
    Ammo, Cooldown, EquippedBy, Item, ItemFootprint, ItemKey, ItemLabel, ItemViewDefinition,
    ItemViewRegistry, OnGround, Shootable, StoredIn, build_chrome_patch,
};

pub struct MagnumPlugin;

impl Plugin for MagnumPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Magnum>()
            .add_systems(Startup, register_magnum_view);
    }
}

pub const MAGNUM_KEY: &str = "core::item::magnum";

#[derive(SceneComponent, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Magnum;

impl Magnum {
    pub fn scene() -> impl Scene {
        bsn! {
            Item {
                key: {ItemKey(MAGNUM_KEY.to_string())},
                label: {ItemLabel("Magnum".to_string())},
                footprint: {ItemFootprint(UVec2 { x: 4, y: 8 })} // BFG of the long variety
            }
            Shootable { cooldown: {Cooldown(0.5)}, magazine_size: 6 }
            Ammo(12) // 2 Magazines
            Visibility
        }
    }
}

const COLOR: Color = Color::srgb(0.82, 0.78, 0.72);
const ICON_COLOR: Color = Color::WHITE;
const ICON_BORDER_COLOR: Color = Color::BLACK;
const ICON_BORDER_PX: f32 = 2.0;

fn register_magnum_view(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut patches: ResMut<Assets<ScenePatch>>,
    mut views: ResMut<ItemViewRegistry>,
) {
    // Grounded and hand meshes are different to emphasize how
    // the `Rusty` component can be applied in any state of the mesh
    let ground_mesh = meshes.add(Cuboid::new(0.1, 0.15, 0.35));
    let hand_mesh = meshes.add(Sphere::new(0.1));

    let material = materials.add(StandardMaterial::from(COLOR));

    let ground_material = material.clone();
    let ground = build_chrome_patch(
        &asset_server,
        &mut patches,
        bsn! {
            Mesh3d(ground_mesh)
            MeshMaterial3d<StandardMaterial>(ground_material)
        },
    );

    let equipped = build_chrome_patch(
        &asset_server,
        &mut patches,
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
        &mut patches,
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
        ItemKey(MAGNUM_KEY.to_string()),
        ItemViewDefinition {
            chrome: HashMap::from([
                (OnGround::KEY.to_string(), ground),
                (EquippedBy::KEY.to_string(), equipped),
                (StoredIn::KEY.to_string(), stored),
            ]),
        },
    );
}
