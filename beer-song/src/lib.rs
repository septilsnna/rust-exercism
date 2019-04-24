pub fn verse(n: i32) -> String {
    let mut v1 = String::new();
    let num = n - 1;
    match n {
        0 => v1.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => v1.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => v1.push_str(&String::from(n.to_string()+" bottles of beer on the wall, "+&n.to_string()+" bottles of beer.\n"+"Take one down and pass it around, "+&num.to_string()+" bottle of beer on the wall.\n")),
        3...99 => v1.push_str(&String::from(n.to_string()+" bottles of beer on the wall, "+&n.to_string()+" bottles of beer.\n"+"Take one down and pass it around, "+&num.to_string()+" bottles of beer on the wall.\n")),
        _ => v1.push_str(""),
    }
    v1
}

pub fn sing(start: i32, end: i32) -> String {
    let mut v1 = String::new();
    for n in end..(start+1) {
        let num = n - 1;
        match n {
            0 => v1.insert_str(0, "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n\n"),
            1 => v1.insert_str(0, "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n"),
            2 => v1.insert_str(0, &String::from(n.to_string()+" bottles of beer on the wall, "+&n.to_string()+" bottles of beer.\n"+"Take one down and pass it around, "+&num.to_string()+" bottle of beer on the wall.\n\n")),
            3...99 => v1.insert_str(0, &String::from(n.to_string()+" bottles of beer on the wall, "+&n.to_string()+" bottles of beer.\n"+"Take one down and pass it around, "+&num.to_string()+" bottles of beer on the wall.\n\n")),
            _ => v1.insert_str(0, ""),
        }
    }
    v1.pop();
    v1
}
