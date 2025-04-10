pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    Pyramid,
    Parallelepiped,
}

pub fn square_area(side: usize) -> usize {
    side.pow(2)
}
pub fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}
