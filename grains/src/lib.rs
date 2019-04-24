pub fn square(s: u32) -> u64 {
    if s <= 0 || s >= 65{
        panic!("Square must be between 1 and 64");
    }
    (2 as u64).pow(s-1)
    //unimplemented!("grains of rice on square {}", s);
}

pub fn total() -> u64 {
    let mut kotak = (1..65).collect::<Vec<u64>>();
    for i in 0..64{
        kotak[i] = (2 as u64).pow(i as u32);
    }
    kotak.iter().sum()
    //unimplemented!();
}
