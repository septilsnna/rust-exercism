pub struct Triangle{
    a : u64,
    b : u64,
    c : u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sum: u64 = 0;
        let mut max: u64 = 0;
        for x in sides.iter() {
            if *x > max {
                max = *x
            }
            sum += *x
        }
        if max > (sum/2) || max == 0 {
            None
        } else {
            Some(Triangle{a : sides[0], b : sides[1], c : sides[2]})
        }
    }

    pub fn is_equilateral(&self) -> bool {
        if &self.a == &self.b && &self.b == &self.c {
            true
        } else {
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        if &self.a != &self.b && &self.b != &self.c && &self.a != &self.c {
            true
        } else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        if &self.a == &self.b && &self.b != &self.c {
            true
        } else if &self.b == &self.c && &self.a != &self.c {
            true
        } else if &self.a == &self.c && &self.a != &self.b {
            true
        } else {
            false
        }
    }
}