pub struct Dictionary{
    phonems:Vec<String>,
    endings:Vec<String>,
}
pub fn longest_matching(a:&[u8], b:&[u8])->usize{
    let c = a[0 as usize];
    let mut count = 0_usize;
    while b[count] != c{
        count +=1;
        if count>=b.len(){
            return 0;
        }
    }
    let mut i = 0_usize;
    while a[i] == b[count+i]{
        i += 1;
        if i>=a.len() || count+i >=b.len(){
            break;
        }
    }
    return i;
}
pub fn ends_with(word:&[u8], value:&[u8])->bool{
    let mut disp = 1;
    while word[word.len()-disp] == value[value.len()-disp]{
        disp += 1;
        if value.len()-disp<0 || word.len()-disp<0{
            return true;
        }
    }
    return false;
}
pub fn make_dictionary(examples:&[&str])->Dictionary{
    let mut phons:Vec<String> = vec![];
    todo!()

}   