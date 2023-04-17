use std::{
    cmp,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vector3Int {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3Int {
    pub const UP: Vector3Int = Vector3Int { x: 0, y: 1, z: 0 };
    pub const DOWN: Vector3Int = Vector3Int { x: 0, y: -1, z: 0 };
    pub const LEFT: Vector3Int = Vector3Int { x: -1, y: 0, z: 0 };
    pub const RIGHT: Vector3Int = Vector3Int { x: 1, y: 0, z: 0 };
    pub fn new(x: i32, y: i32, z: i32) -> Vector3Int {
        Vector3Int { x, y, z }
    }
    pub fn manhattan(&self, other: Vector3Int) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Vector3Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector3Int::new(
            self.x + other.x,
            self.y + other.y,
            cmp::max(self.z, other.z), // We take the highest z-index.
        );
    }
}

impl AddAssign for Vector3Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: cmp::max(self.z, other.z),
        };
    }
}

impl Sub for Vector3Int {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector3Int::new(
            self.x - other.x,
            self.y - other.y,
            cmp::max(self.z, other.z),
        );
    }
}

impl SubAssign for Vector3Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: cmp::max(self.z, other.z),
        };
    }
}

impl Div<i32> for Vector3Int {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        return Vector3Int::new(self.x / other, self.y / other, self.z);
    }
}

impl Mul<i32> for Vector3Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Vector3Int::new(self.x * other, self.y * other, self.z);
    }
}

impl Mul<Vector3Int> for i32 {
    type Output = Vector3Int;

    fn mul(self, other: Vector3Int) -> Vector3Int {
        return Vector3Int::new(other.x * self, other.y * self, other.z);
    }
}

pub const ORTHO_DIRECTIONS: [Vector3Int; 4] = [
    Vector3Int::UP,
    Vector3Int::DOWN,
    Vector3Int::LEFT,
    Vector3Int::RIGHT,
];
