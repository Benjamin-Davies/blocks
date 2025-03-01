use glam::{IVec3, Vec3};

use blocks_game::{
    bounding_box::BoundingBox,
    terrain::{block::Block, Terrain},
    util::TotalOrd,
};

const MAX_REACH: f32 = 10.0;
const EPSILON: f32 = 0.0001;

/// Finds the block that the player is clicking on. Returns a tuple of the block position and the
/// direction of the face that was clicked.
pub fn ray_cast(origin: Vec3, direction: Vec3, terrain: &Terrain) -> Option<(IVec3, IVec3)> {
    let bounding_box = BoundingBox::new(
        origin - Vec3::splat(MAX_REACH),
        origin + Vec3::splat(MAX_REACH),
    );

    terrain
        .blocks_intersecting(bounding_box)
        .filter(|&(_, b)| b != Block::AIR)
        .filter_map(|(p, _)| intersect_block(origin, direction, p).and_then(|r| Some((p, r))))
        .min_by_key(|&(_, r)| TotalOrd(r))
        .filter(|&(p, _)| p.as_vec3().distance_squared(origin) <= MAX_REACH * MAX_REACH)
        .map(|(p, r)| {
            (
                p,
                direction_of(origin + r * direction - (p.as_vec3() + Vec3::splat(0.5))),
            )
        })
}

fn intersect_block(origin: Vec3, direction: Vec3, pos: IVec3) -> Option<f32> {
    let (t_x_min, t_x_max) =
        intersect_axis(origin.x, direction.x, pos.x as f32, pos.x as f32 + 1.0);
    let (t_y_min, t_y_max) =
        intersect_axis(origin.y, direction.y, pos.y as f32, pos.y as f32 + 1.0);
    let (t_z_min, t_z_max) =
        intersect_axis(origin.z, direction.z, pos.z as f32, pos.z as f32 + 1.0);

    let t_min = t_x_min.max(t_y_min).max(t_z_min);
    let t_max = t_x_max.min(t_y_max).min(t_z_max);

    if t_min <= t_max && t_max >= 0.0 {
        Some(t_min)
    } else {
        None
    }
}

fn intersect_axis(origin: f32, direction: f32, min: f32, max: f32) -> (f32, f32) {
    if direction.abs() < EPSILON {
        if origin >= min && origin <= max {
            return (f32::NEG_INFINITY, f32::INFINITY);
        }
        return (f32::INFINITY, f32::NEG_INFINITY);
    }

    let t0 = (min - origin) / direction;
    let t1 = (max - origin) / direction;

    if direction < 0.0 {
        (t1, t0)
    } else {
        (t0, t1)
    }
}

/// Returns the cardinal direction that most closely matches the given vector.
fn direction_of(v: Vec3) -> IVec3 {
    [
        IVec3::X,
        IVec3::NEG_X,
        IVec3::Y,
        IVec3::NEG_Y,
        IVec3::Z,
        IVec3::NEG_Z,
    ]
    .into_iter()
    .max_by_key(|d| TotalOrd(v.dot(d.as_vec3())))
    .unwrap()
}
