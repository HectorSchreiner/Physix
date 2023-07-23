
pub mod physix {

type real = f32;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: real,
    pub y: real,
    pub z: real
}

impl Vector3 {
    pub fn new(x: real, y: real, z: real) -> Self {
        Self { x, y, z }
    }

    pub fn invert(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn len(self) -> real {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_len(self) -> real {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normal(&mut self) {
        let l: real = self.len();
        if l > 0.0 {
            self.x = self.x / l;
            self.y = self.y / l;
            self.z = self.z / l;
        }
    }

    
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

}