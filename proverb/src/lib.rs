pub fn build_proverb(list: &[&str]) -> String {
    let mut string: String = String::new();
    if list.len()==0 {return string}
    for i in 0 ..(list.len()-1){
        string += &format!("For want of a {} the {} was lost.\n",list[i],list[i+1]);
    }
    string += &format!("And all for the want of a {}.", list[0]);
    string
    //unimplemented!("build a proverb from this list of items: {:?}", list)
}
