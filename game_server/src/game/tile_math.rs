use crate::game::math::Vec2F;

pub const TILE_SIZE: f32 = 1.0;

pub const fn align_to_tile(position: f32) -> f32 {
    let int_part = position as i32;
    if position >= 0.0 {
        int_part as f32
    } else if (int_part as f32) == position {
        position // already aligned
    } else {
        (int_part - 1) as f32 // move to lower tile
    }
}

pub const fn align_vec2f_to_tile(v: Vec2F) -> Vec2F {
    Vec2F::new(align_to_tile(v.x),align_to_tile(v.y))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_align_to_tile_non_negative() {
        assert_eq!(TILE_SIZE, 1.0);

        assert_eq!(align_to_tile(2.4), 2.0);

        assert_eq!(align_to_tile(0.0), 0.0);

        let v1 = Vec2F::new(1.1, 0.0);
        assert_eq!(align_vec2f_to_tile(v1), Vec2F::new(1.0, 0.0));

        let v2 = Vec2F::new(1.0, 0.0);
        assert_eq!(align_vec2f_to_tile(v2), Vec2F::new(1.0, 0.0));

        let v3 = Vec2F::new(0.1, 0.0);
        assert_eq!(align_vec2f_to_tile(v3), Vec2F::new(0.0, 0.0));

        let v4 = Vec2F::new(0.0, 0.0);
        assert_eq!(align_vec2f_to_tile(v4), Vec2F::new(0.0, 0.0));
    }
    #[test]
    fn test_align_to_tile_negative() {
        assert_eq!(align_to_tile(-5.2), -6.0);

        assert_eq!(align_to_tile(-0.2), -1.0);

        let v1 = Vec2F::new(-0.1, 0.0);
        assert_eq!(align_vec2f_to_tile(v1), Vec2F::new(-1.0, 0.0));

        let v2 = Vec2F::new(-0.6, 0.0);
        assert_eq!(align_vec2f_to_tile(v2), Vec2F::new(-1.0, 0.0));

        let v3 = Vec2F::new(-1.0, 0.0);
        assert_eq!(align_vec2f_to_tile(v3), Vec2F::new(-1.0, 0.0));

        let v4 = Vec2F::new(-1.1, 0.0);
        assert_eq!(align_vec2f_to_tile(v4), Vec2F::new(-2.0, 0.0));
    }
}