
pub mod physix {

pub type real = f32;

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

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    // creates a new object equal to the crossproduct.
    pub fn cross_product(self, vec: Self) -> Self {
        Self {  
            x: self.y * vec.z - self.z * vec.y, 
            y: self.x * vec.z - self.z * vec.x, 
            z: self.y * vec.x - self.x * vec.y
        }
    }

    // sets current vec equal to croosproduct.
    pub fn cross_product_eq(&mut self, vec: Self) {
        self.x = self.y * vec.z - self.z * vec.y;
        self.z = self.x * vec.z - self.z * vec.x;
        self.y = self.y * vec.x - self.x * vec.y;
    }

    //  scales a vector and returns a new updated vector
    pub fn add_scaled_vector(&mut self, vector: Vector3, scale: real) {
        self.x += vector.x * scale;
        self.y += vector.y * scale;
        self.z += vector.z * scale;
    }
 
    pub fn dot_product(self, other: Self) -> real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mul_assign(&mut self, value: f32) {
        self.x *= value;
        self.y *= value;
        self.z *= value;
    }
        
    pub fn mul(self, value: real) -> Self {
        Vector3::new(self.x * value, self.y * value, self.z * value)
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

