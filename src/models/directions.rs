pub struct Directions {
    z: i32,
    q: i32,
    s: i32,
    d: i32,
}

impl Directions {
    pub fn new(z: i32, q: i32, s: i32, d: i32) -> Self {
        Directions { z, q, s, d }
    }

    pub fn get_z(&self) -> i32 {
        self.z
    }

    pub fn get_q(&self) -> i32 {
        self.q
    }

    pub fn get_s(&self) -> i32 {
        self.s
    }

    pub fn get_d(&self) -> i32 {
        self.d
    }
}