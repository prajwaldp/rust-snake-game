#[derive(Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(PartialEq)]
pub enum Status {
    NORMAL,
    OVER,
}
