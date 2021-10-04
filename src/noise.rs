

#![allow(non_camel_case_types)]
pub trait NoiseT {
    type BaseT;

    fn set_seed(&mut self, seed: Self::BaseT);
    fn get_seed(&self) -> Self::BaseT;
    fn set_bit_noise(&mut self, bn1: Self::BaseT, bn2: Self::BaseT, bn3: Self::BaseT);
    fn set_primes(&mut self, p1: Self::BaseT, p2: Self::BaseT, p3: Self::BaseT);

    fn get_1d(&self, x: Self::BaseT) -> Self::BaseT;
    fn get_2d(&self, x: Self::BaseT, x: Self::BaseT) -> Self::BaseT;
    fn get_3d(&self, x: Self::BaseT, y: Self::BaseT, z: Self::BaseT) -> Self::BaseT;
    fn get_4d(&self, x: Self::BaseT, y: Self::BaseT, z: Self::BaseT, t: Self::BaseT) -> Self::BaseT;
}

pub struct Squirrel<T> {
    seed: T,
    bit_noise1: T,
    bit_noise2: T,
    bit_noise3: T,
    y_prime: T,
    z_prime: T,
    t_prime: T,
}

type Squirrel_U32 = Squirrel<u32>;

impl Squirrel_U32 {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            bit_noise1: 123456187,
            bit_noise2: 98765549,
            bit_noise3: 5279501,
            y_prime: 8132429,
            z_prime: 346309,
            t_prime: 0,
        }
    }
}

impl NoiseT for Squirrel_U32 {
    type BaseT = u32;

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed
    }

    fn get_seed(&self) -> u32 {
        self.seed
    }

    fn set_bit_noise(&mut self, bn1: u32, bn2: u32, bn3: u32) {
        self.bit_noise1 = bn1;
        self.bit_noise2 = bn2;
        self.bit_noise3 = bn3;
    }

    fn set_primes(&mut self, p1: u32, p2: u32, p3: u32) {
        self.y_prime = p1;
        self.z_prime = p2;
        self.t_prime = p3;
    }

    fn get_1d(&self, x: u32) -> u32 {
        let mut mangled = x;
        mangled *= self.bit_noise1;
        mangled += self.seed;
        mangled ^= mangled >> 8;
        mangled += self.bit_noise2;
        mangled ^= mangled << 8;
        mangled *= self.bit_noise3;
        mangled ^= mangled >> 8;
        mangled
    }

    fn get_2d(&self, x: u32, y: u32) -> u32 {
        let new_x = x + (self.y_prime * y);
        self.get_1d(new_x)
    }

    fn get_3d(&self, x: u32, y: u32, z: u32) -> u32 {
        let new_x = x + (self.y_prime * y) + (self.z_prime * z);
        self.get_1d(new_x)
    }

    fn get_4d(&self, x: u32, y: u32, z: u32, t: u32) -> u32 {
        let new_x = x + (self.y_prime * y) + (self.z_prime * z) + (self.t_prime * t);
        self.get_1d(new_x)
    }
}

pub trait RandomT {
    type BaseT;

    fn set_seed(&mut self, seed: Self::BaseT);
    fn get_seed(&self) -> Self::BaseT;
    fn set_pos(&mut self, pos: Self::BaseT);
    fn get_pos(&self) -> Self::BaseT;
    fn get_bool(&mut self) -> bool;
    fn get_u8(&mut self) -> u8;
    fn get_u16(&mut self) -> u16;
    fn get_u32(&mut self) -> u32;
    fn get_u64(&mut self) -> u64;
    fn get_f32(&mut self) -> f32;
    fn get_f64(&mut self) -> f64;
}

pub struct RandomNoise<T, U> {
    pos: T,
    noise: U,
}

type RD_Sq_U32 = RandomNoise<u32, Squirrel_U32>;

impl RD_Sq_U32 {
    pub fn new(noise: Squirrel_U32) -> Self {
        Self {
            pos: 0,
            noise,
        }
    }
}

impl RandomT for RD_Sq_U32 {
    type BaseT = u32;

    fn set_seed(&mut self, seed: u32) {
        self.noise.set_seed(seed)
    }

    fn get_seed(&self) -> u32 {
        self.noise.get_seed()
    }

    fn set_pos(&mut self, pos: u32) {
        self.pos = pos
    }

    fn get_pos(&self) -> u32 {
        self.pos
    }

    fn get_bool(&mut self) -> bool {
        todo!()
    }

    fn get_u8(&mut self) -> u8 {
        todo!()
    }

    fn get_u16(&mut self) -> u16 {
        todo!()
    }

    fn get_u32(&mut self) -> u32 {
        todo!()
    }

    fn get_u64(&mut self) -> u64 {
        todo!()
    }

    fn get_f32(&mut self) -> f32 {
        todo!()
    }

    fn get_f64(&mut self) -> f64 {
        todo!()
    }
}
