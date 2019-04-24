pub fn raindrops(n: u32) -> String {
    let mut x = false;
    let mut string = String::new();
    if ((n%3)==0){
        x = true;
        string.push_str("Pling");
    }
    if ((n%5)==0){
        x = true;
        string.push_str("Plang");
    }
    if ((n%7)==0){
        x = true;
        string.push_str("Plong");
    }
    if x{
        return string;
    }
    n.to_string()

    //unimplemented!("what sound does Raindrop #{} make?", n)
}
