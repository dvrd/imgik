
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }

    pub fn black() -> Rgb {
        Rgb { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Rgb {
        Rgb { r: 1.0, g: 1.0, b: 1.0 }
    }

    pub fn red() -> Rgb {
        Rgb { r: 1.0, g: 0.0, b: 0.0 }
    }

    pub fn green() -> Rgb {
        Rgb { r: 0.0, g: 1.0, b: 0.0 }
    }

    pub fn blue() -> Rgb {
        Rgb { r: 0.0, g: 0.0, b: 1.0 }
    }

    pub fn yellow() -> Rgb {
        Rgb { r: 1.0, g: 1.0, b: 0.0 }
    }

    pub fn cyan() -> Rgb {
        Rgb { r: 0.0, g: 1.0, b: 1.0 }
    }

    pub fn magenta() -> Rgb {
        Rgb { r: 1.0, g: 0.0, b: 1.0 }
    }

    pub fn gray() -> Rgb {
        Rgb { r: 0.5, g: 0.5, b: 0.5 }
    }

    pub fn as_red(&self) -> Rgb {
        Rgb { r: self.r, g: 0.0, b: 0.0 }
    }

    pub fn mean(&self) -> Rgb {
        let mean = (self.r + self.g + self.b) / 3.0;
        Rgb {
            r: mean,
            g: mean,
            b: mean,
        }
    }

    pub fn quantize(&self) -> Rgb {
        Rgb {
            r: self.r.round(),
            g: self.g.round(),
            b: self.b.round(),
        }
    }

    pub fn to_u8(&self) -> Vec<u8> {
        vec![
            (255.0 * self.r) as u8, 
            (255.0 * self.g) as u8, 
            (255.0 * self.b) as u8
        ]
    }

    pub fn invert(&self) -> Rgb {
        Rgb {
            r: 1.0 - self.r,
            g: 1.0 - self.g,
            b: 1.0 - self.b,
        }
    }
}

impl From<&[u8]> for Rgb {
    fn from(v: &[u8]) -> Rgb {
        Rgb {
            r: v[0] as f32 / 255.0,
            g: v[1] as f32 / 255.0,
            b: v[2] as f32 / 255.0,
        }
    }
}

