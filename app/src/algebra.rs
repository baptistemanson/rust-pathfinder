pub type Vec2 = [f32; 2];
pub struct Box {
    pub top_left: Vec2,
    pub top_right: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
}

pub fn div(first: Vec2, dim: Vec2) -> Vec2 {
    [first[0] / dim[0], first[1] / dim[1]]
}
#[allow(dead_code)]
pub fn mul(first: Vec2, dim: Vec2) -> Vec2 {
    [first[0] * dim[0], first[1] * dim[1]]
}

impl Box {
    pub fn div(&self, dim: Vec2) -> Box {
        Box {
            top_left: div(self.top_left, dim),
            top_right: div(self.top_right, dim),
            bottom_left: div(self.bottom_left, dim),
            bottom_right: div(self.bottom_right, dim),
        }
    }
    #[allow(dead_code)]
    pub fn mul(&self, dim: Vec2) -> Box {
        Box {
            top_left: mul(self.top_left, dim),
            top_right: mul(self.top_right, dim),
            bottom_left: mul(self.bottom_left, dim),
            bottom_right: mul(self.bottom_right, dim),
        }
    }
}
// scale factor is applied to dim.
pub fn make_box([x, y]: [f32; 2], [width, height]: [f32; 2]) -> Box {
    let top_left = [x, y];
    let bottom_right = [x + width, y + height];
    let top_right = [bottom_right[0], top_left[1]];
    let bottom_left = [top_left[0], bottom_right[1]];
    Box {
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    }
}
