use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_event::<CubeSelectionEvent>()
        // .add_system(selected_listener)
        .add_system(spawn_cube)
        .add_system(move_cubes)
        .add_system(select_cube)
        .add_system(handle_selection_color_switch)
        .run()
}

#[derive(Component)]
pub struct MyCubeComponent;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SpinOffset(f32);

#[derive(Resource)]
pub struct MyHandles {
    mesh: Handle<Mesh>,
    gray_material: Handle<StandardMaterial>,
    red_material: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(40., 20., 50.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // cube mesh
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 5. }));

    // cube mats
    let gray_material = materials.add(StandardMaterial::default());
    let red_material = materials.add(Color::rgb(255., 0., 0.).into());

    // spawn a cube
    let cube_entity = commands
        .spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: red_material.clone(),
                ..default()
            },
            MyCubeComponent,
            SpinOffset(0.),
            Selected,
        ))
        .id();

    commands.insert_resource(MyHandles {
        mesh,
        gray_material,
        red_material,
    });

    commands.insert_resource(CubeSelection {
        order: vec![cube_entity],
        idx: 0,
    });

    // spawn a light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn spawn_cube(
    mut commands: Commands,
    mut cube_sel: ResMut<CubeSelection>,
    my_cube: Res<MyHandles>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut rnd = thread_rng();
    if keyboard.just_pressed(KeyCode::Space) {
        // spawn a cube
        let entity = commands
            .spawn((
                PbrBundle {
                    mesh: my_cube.mesh.clone(),
                    material: my_cube.gray_material.clone(),
                    transform: Transform::from_xyz(
                        rnd.gen_range(-20.0..20.),
                        rnd.gen_range(-10.0..10.),
                        0.,
                    ),
                    ..default()
                },
                MyCubeComponent,
                SpinOffset(rnd.gen()),
            ))
            .id();
        cube_sel.order.push(entity)
    }
}

fn move_cubes(
    mut cubes_query: Query<(&mut Transform, &SpinOffset), With<MyCubeComponent>>,
    time: Res<Time>,
) {
    for (mut transform, spin_offset) in &mut cubes_query {
        let time = time.elapsed_seconds() * (10. + spin_offset.0);
        transform.translation.z += time.sin();
        transform.translation.x += time.cos();
    }
}

#[derive(Resource)]
struct CubeSelection {
    order: Vec<Entity>,
    idx: usize,
}

// Query solution (Probably most optimized because of query optimization)
fn handle_selection_color_switch(
    mut selected: Query<&mut Handle<StandardMaterial>, With<Selected>>,
    mut not_selected: Query<&mut Handle<StandardMaterial>, Without<Selected>>,
    my_handles: Res<MyHandles>,
) {
    for mut mat in &mut selected {
        *mat = my_handles.red_material.clone()
    }

    for mut mat in &mut not_selected {
        *mat = my_handles.gray_material.clone()
    }
}

fn select_cube(
    mut commands: Commands,
    mut selection: ResMut<CubeSelection>,
    // mut color_event: EventWriter<CubeSelectionEvent>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut direction = 0;

    if keyboard.just_pressed(KeyCode::Left) && selection.idx > 0 {
        direction = -1;
    } else if keyboard.just_pressed(KeyCode::Right) && selection.idx < selection.order.len() - 1 {
        direction = 1;
    }

    if direction != 0 {
        // Remove Selected from the currently selected cube
        commands
            .entity(selection.order[selection.idx])
            .remove::<Selected>();

        // Trigger color change event for switching back to gray
        // color_event.send(CubeSelectionEvent {
        //     cube: selection.order[selection.idx],
        //     is_selected: false,
        // });

        // Update the index
        selection.idx = (selection.idx as isize + direction) as usize;

        // Add selected to the cube at the new index
        let new_selected = selection.order[selection.idx];
        commands.entity(new_selected).insert(Selected);

        // Trigger color change event for switching to red
        // color_event.send(CubeSelectionEvent {
        //     cube: new_selected,
        //     is_selected: true,
        // });
    }
}

// pub struct CubeSelectionEvent {
//     cube: Entity,
//     is_selected: bool,
// }
//
// fn selected_listener(
//     mut commands: Commands,
//     mut events: EventReader<CubeSelectionEvent>,
//     my_handles: Res<MyHandles>,
// ) {
//     for event in &mut events {
//         commands
//             .entity(event.cube)
//             .remove::<Handle<StandardMaterial>>();
//         commands.entity(event.cube).insert(match event.is_selected {
//             true => my_handles.red_material.clone(),
//             false => my_handles.gray_material.clone(),
//         });
//     }
// }
