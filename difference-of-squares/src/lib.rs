pub fn square_of_sum(n: u32) -> u32 {
   let mut m = 0;
    for i in 1..(n+1) {
        m = m+i;
    }
    m = m * m;
    m
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut m = 0;
    for i in 1..(n+1) {
        let o = i * i;
        m = m+o;
    }
    m
}

pub fn difference(n: u32) -> u32 {
    let m = square_of_sum(n)-sum_of_squares(n);
    m
}
