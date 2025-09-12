mod areas_volumes;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};
pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area = (x * y) as f64;

    let shape_area = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
    };

    shape_area * times as f64 <= area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume = (x * y * z) as f64;

    let shape_volume = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };

    shape_volume * times as f64 <= volume
}
