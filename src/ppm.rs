// PPM 

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", &self.r, &self.g, &self.b)
    }
}

pub struct PPM {
    width: u16,
    height: u16,
    data: Vec<Vec<Pixel>>
}

pub fn row_to_string(row: &Vec<Pixel>) -> String {
    let mut out: String = String::new();
    for x in row {
        out.push_str(x.to_string().as_str());
        out.push_str(" ");
    }
    out
}

impl PPM {
    pub fn New(width: u16, height: u16) -> PPM {
        PPM {
            width: width,
            height: height,
            data: vec![vec![Pixel { r: 255, g: 255, b: 255};height as usize]; width as usize]
        }
    }
    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn to_string(&self) -> String {
        let mut s: String;
        s = format!("P3\n{} {}\n255\n", self.width, self.height);
        for x in &self.data {
            s.push_str(row_to_string(x).as_str());
            s.pop();
            s.push_str("\n");
        }
        s.pop();
        s
    }
}

