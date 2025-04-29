// Adapted from
// https://github.com/georust/wkb/blob/main/src/test/data.rs

use geo::{MultiPoint, Point, point};

pub(super) fn point_2d() -> Point {
    point!(
        x: -3.7038, y: 40.4168
    )
}

// pub(super) fn linestring_2d() -> LineString {
//     line_string![
//         (x: -73.935242, y: 40.730610), // New York City
//         (x: -118.243683, y: 34.052235) // Los Angeles
//     ]
// }

// pub(super) fn polygon_2d() -> Polygon {
//     polygon![
//         (x: -122.419418, y: 37.774929), // San Francisco
//         (x: -122.419418, y: 34.052235), // Los Angeles
//         (x: -118.243683, y: 34.052235), // Los Angeles
//         (x: -118.243683, y: 37.774929), // San Francisco
//     ]
// }

// pub(super) fn polygon_2d_with_interior() -> Polygon {
//     polygon!(
//         exterior: [
//             (x: -122.419418, y: 37.774929), // San Francisco
//             (x: -122.419418, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 37.774929), // San Francisco
//         ],
//         interiors: [
//             [
//                 (x: -121.886330, y: 37.338207), // San Jose
//                 (x: -121.886330, y: 36.778259), // Central California
//                 (x: -119.417931, y: 36.778259), // Central California
//                 (x: -119.417931, y: 37.338207), // San Jose
//             ],
//         ],
//     )
// }

pub(super) fn multi_point_2d() -> MultiPoint {
    MultiPoint::new(vec![
        point!(
            x: -3.7038, y: 40.4168
        ),
        point!(
            x: 33.75, y: -11.178401873711776
        ),
    ])
}

// pub(super) fn multi_line_string_2d() -> MultiLineString {
//     MultiLineString::new(vec![
//         line_string![
//             (x: -122.419418, y: 37.774929), // San Francisco
//             (x: -122.419418, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 37.774929), // San Francisco
//         ],
//         line_string![
//             (x: -121.886330, y: 37.338207), // San Jose
//             (x: -121.886330, y: 36.778259), // Central California
//             (x: -119.417931, y: 36.778259), // Central California
//             (x: -119.417931, y: 37.338207), // San Jose
//         ],
//     ])
// }

// pub(super) fn multi_polygon_2d() -> MultiPolygon {
//     MultiPolygon::new(vec![
//         polygon![
//             (x: -122.419418, y: 37.774929), // San Francisco
//             (x: -122.419418, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 34.052235), // Los Angeles
//             (x: -118.243683, y: 37.774929), // San Francisco
//         ],
//         polygon!(
//             exterior: [
//                 (x: -122.419418, y: 37.774929), // San Francisco
//                 (x: -122.419418, y: 34.052235), // Los Angeles
//                 (x: -118.243683, y: 34.052235), // Los Angeles
//                 (x: -118.243683, y: 37.774929), // San Francisco
//             ],
//             interiors: [
//                 [
//                     (x: -121.886330, y: 37.338207), // San Jose
//                     (x: -121.886330, y: 36.778259), // Central California
//                     (x: -119.417931, y: 36.778259), // Central California
//                     (x: -119.417931, y: 37.338207), // San Jose
//                 ],
//             ],
//         ),
//     ])
// }

// pub(super) fn geometry_collection_2d() -> GeometryCollection {
//     GeometryCollection::new_from(vec![
//         Geometry::Point(point_2d()),
//         Geometry::LineString(linestring_2d()),
//         Geometry::Polygon(polygon_2d()),
//         Geometry::Polygon(polygon_2d_with_interior()),
//         Geometry::MultiPoint(multi_point_2d()),
//         Geometry::MultiLineString(multi_line_string_2d()),
//         Geometry::MultiPolygon(multi_polygon_2d()),
//     ])
// }
