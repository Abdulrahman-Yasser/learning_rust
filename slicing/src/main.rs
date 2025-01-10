fn main() {
    let mut s = String::from("hello world");
    let s2 = "hello world";
    let hello = &s[..5];
    let world = &s[6..];

    let word = new_first_word(&s2);
}


fn first_word(s:&String) ->usize{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

 
fn new_first_word(s:&str) ->&str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
