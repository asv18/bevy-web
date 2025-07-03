use bevy::prelude::*;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! {
        <p>
            "wassup"
        </p>
    });

    wasm_bindgen_futures::spawn_local(async {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#bevy".into()),
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, setup)
            // .add_plugins(HelloPlugin)
        .run();
    });
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


// fn hello_world() {
//     println!("Hello, world!");
// }

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands.spawn(
//         (Person, Name("placeholder".to_string()))
//     );
// }

// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("Hello, {}!", name.0);
//     }
// }

// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "placeholder" {
//             name.0 = "Blank".to_string();
//             break;
//         }
//     }
// }

// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, add_people);
//         app.add_systems(Update, (hello_world, (update_people, greet_people)));
//     }
// }