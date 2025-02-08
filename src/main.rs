mod camera_controller;
use bevy::asset::RenderAssetUsages;
use bevy::pbr::ShadowFilteringMethod;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use camera_controller::{CameraController, CameraControllerPlugin};
use shapefile::{PointZ, Reader, Shape};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let argv: Vec<String> = std::env::args().collect();
    println!("{:?}", argv);
    let path = argv.get(1).expect("shapefile path not given");
    let pos_x: f64 = argv
        .get(2)
        .expect("pos_x not given")
        .parse()
        .expect("pos_x is not float type");
    let pos_y: f64 = argv
        .get(3)
        .expect("pos_y not given")
        .parse()
        .expect("pos_y is not float type");
    // polygons
    let pos_0 = [pos_x, pos_y];
    let mut reader = Reader::from_path(path).unwrap();
    let polygons: Vec<Vec<PointZ>> = reader
        .iter_shapes_and_records()
        .map(|row| {
            let polyz = match row {
                Ok((Shape::PolygonZ(polyz), _)) => polyz,
                Ok((shape, _)) => panic!("{shape} is not PolygonZ"),
                Err(e) => panic!("{}", e),
            };
            let points = match polyz.rings() {
                [polyzring] => polyzring.points(),
                other => panic!("{other:?} exists more than one ring"),
            };
            points
                .iter()
                .map(|pointz| {
                    let mut pointz = pointz.to_owned();
                    pointz.x -= pos_0[0];
                    pointz.y -= pos_0[1];
                    (pointz.x, pointz.y) = (pointz.y, pointz.x);
                    pointz
                })
                .collect()
        })
        .collect();
    commands.spawn((
        Mesh3d(meshes.add(create_buildings(&polygons))),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: LinearRgba::rgb(154. / 225., 184. / 225., 255. / 225.),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(create_stroke(&polygons))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 35000.0,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 1000.0, 0.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::new(-1.0, 1.0, 0.0), Vec3::Y),
        CameraController::default(),
        ShadowFilteringMethod::Hardware2x2,
    ));
}

fn create_stroke(polygons: &[Vec<PointZ>]) -> Mesh {
    let res = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            polygons
                .iter()
                .flat_map(|polygon| {
                    polygon.windows(2).flat_map(|line| {
                        let (p1, p2) = (line[0], line[1]);
                        [
                            [p1.x, 0.0, p1.y],
                            [p1.x, p1.z, p1.y],
                            [p1.x, p1.z, p1.y],
                            [p2.x, p2.z, p2.y],
                        ]
                    })
                })
                .map(|[x, y, z]| [x as f32, y as f32, z as f32])
                .collect::<Vec<_>>(),
        );
    res
}

fn create_buildings(polygons: &[Vec<PointZ>]) -> Mesh {
    let mut res = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        polygons
            .iter()
            .flat_map(|polygon| {
                polygon
                    .windows(2)
                    .flat_map(|line| {
                        let (p1, p2) = (line[0], line[1]);
                        [
                            [p1.x, 0.0, p1.y],
                            [p2.x, 0.0, p2.y],
                            [p2.x, p2.z, p2.y],
                            [p1.x, 0.0, p1.y],
                            [p2.x, p2.z, p2.y],
                            [p1.x, p1.z, p1.y],
                            [p1.x, 0.0, p1.y],
                            [p2.x, p2.z, p2.y],
                            [p2.x, 0.0, p2.y],
                            [p1.x, 0.0, p1.y],
                            [p1.x, p1.z, p1.y],
                            [p2.x, p2.z, p2.y],
                        ]
                    })
                    .map(|[x, y, z]| [x as f32, y as f32, z as f32])
            })
            .collect::<Vec<_>>(),
    )
    .with_inserted_indices(Indices::U32(
        (0..polygons
            .iter()
            .map(|polygon| (polygon.len() as u32 - 1) * 12)
            .sum())
            .collect(),
    ));
    res.compute_normals();
    res
}
