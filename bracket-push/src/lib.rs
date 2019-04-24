pub fn brackets_are_balanced(string: &str) -> bool {
    let mut awal = vec!['0'];
    let buka = vec!['(','{','['];
    let tutup = vec![')','}',']'];
    for i in string.chars(){
        if buka.contains(&i){awal.push(i)}
        else if tutup.contains(&i){
            if &i==&tutup[0] && awal.last()==Some(&buka[0]){awal.pop();}
            else if &i==&tutup[1] && awal.last()==Some(&buka[1]){awal.pop();}
            else if &i==&tutup[2] && awal.last()==Some(&buka[2]){awal.pop();}
            else {return false}
        }
    }
    if buka.contains(&awal[awal.len()-1]){return false}
    true
    //unimplemented!("Check if the string \"{}\" contains balanced brackets",string);
}
