use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ground)
            .add_startup_system(add_blast_zone)
            .add_system(blast_zone_collisions);
    }
}

fn add_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Window>,
) {
    let window = query.single();

    let top = (window.height() / 2.) - 150.;

    // spawn land
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(window.width(), 30.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::hex("c4a484").unwrap())),
        transform: Transform::from_xyz(0., top, 1.),
        ..default()
    });

    // spawn water
    let water_depth = window.height();

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(window.width(), water_depth)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::hex("2063a5").unwrap())),
        transform: Transform::from_xyz(0., top - (water_depth / 2.), 0.),
        ..default()
    });
}

#[derive(Component)]
struct BlastZone {}

fn add_blast_zone(mut commands: Commands, query: Query<&Window>) {
    let window = query.single();

    let window_h = window.height();
    let window_w = window.width();

    let bottom = 0. - (window_h / 2.) - 400.;
    let left = 0. - (window_w / 2.) - 150.;
    let right = (window_w / 2.) + 150.;

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(window_w, 10.))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            Group::from_bits(0b0001).unwrap(),
            Group::from_bits(0b0010).unwrap(),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0., bottom, 1.)))
        .insert(BlastZone {});

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10., window_h))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            Group::from_bits(0b0001).unwrap(),
            Group::from_bits(0b0010).unwrap(),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(right, 0., 1.)))
        .insert(BlastZone {});

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10., window_h))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups::new(
            Group::from_bits(0b0001).unwrap(),
            Group::from_bits(0b0010).unwrap(),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(left, 0., 1.)))
        .insert(BlastZone {});
}

fn blast_zone_collisions(
    mut commands: Commands,
    rap_ctx: Res<RapierContext>,
    mut query: Query<Entity, With<BlastZone>>,
) {
    for bz in query.iter_mut() {
        for contact_pair in rap_ctx.contacts_with(bz) {
            let other_coll = if contact_pair.collider1() == bz {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };

            info!("removing entity");
            commands.entity(other_coll).despawn();
        }
    }
}
