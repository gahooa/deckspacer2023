// create a macro to add_line with assuming a drawing variable is present where macro is invoked
#[macro_export]
macro_rules! add_line {
    ($drawing:ident, $start:expr, $end:expr) => {
        let p1 = dxf::Point::new($start.0, $start.1, 0.0);
        let p2 = dxf::Point::new($end.0, $end.1, 0.0);
        $drawing.add_entity(dxf::entities::Entity::new(dxf::entities::EntityType::Line(
            dxf::entities::Line::new(p1, p2),
        )));
    };
}

#[macro_export]
macro_rules! add_lines {
    ($drawing:ident, $($point:expr),+ $(,)?) => {
        let points: Vec<(f64, f64)> = vec![$($point),+];
        let mut prev = points[0];
        for &p in points.iter().skip(1) {
            add_line!($drawing, prev, p);
            prev = p;
        }
    };
}

#[macro_export]
macro_rules! add_circle {
    ($drawing:ident, $center:expr, $radius:expr) => {
        let p1 = dxf::Point::new($center.0, $center.1, 0.0);
        $drawing.add_entity(dxf::entities::Entity::new(
            dxf::entities::EntityType::Circle(dxf::entities::Circle::new(p1, $radius)),
        ));
    };
}
