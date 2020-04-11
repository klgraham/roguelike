/// A Rectangle with bottom edge (x1, x2) and left edge (y1, y2)
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    /// Creates a Rect with bottom left corner (x, y) of given width and height
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    /// Returns true if this is underneath other
    pub fn is_under(&self, other: &Rect) -> bool {
        self.y2 <= other.y1
    }

    /// Returns true if this is above other
    pub fn is_above(&self, other: &Rect) -> bool {
        self.y1 >= other.y2
    }

    /// Returns true if this is left of other
    pub fn is_left_of(&self, other: &Rect) -> bool {
        self.x2 <= other.x1
    }

    /// Returns true if this is right of other
    pub fn is_right_of(&self, other: &Rect) -> bool {
        self.x1 >= other.x2
    }

    /// Returns true if this overlaps with other
    pub fn overlaps_with(&self, other: &Rect) -> bool {
        !self.is_under(other)
            && !self.is_above(other)
            && !self.is_left_of(other)
            && !self.is_right_of(other)
    }

    /// Returns the center of the rectangle
    // The center is the point made of the midpoints of segments (x1, x2) and (y1, y2)
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
