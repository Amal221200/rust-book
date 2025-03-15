fn main() {
    // let mut s = String::from("hello");

    // let len = len(&s);

    // let r1 = &mut s;
    
    // println!("r1: {}", r1);
    // let r2 = &mut s;
    // println!("r2: {}", r2);
    // println!("Length of string: \"{s}\" is: {}", len);
    let s = String::from("hello world");
    println!("First word of string: \"hello world\" is: {}", second_word(&s));
}

fn _len(str: &str) -> usize {
    str.len()
}

fn _first_word(str: &str) -> &str {
    // let bytes = str.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return &str[0..i];
    //     }
    // }
    // for (s1, s) in str.split(" ").enumerate() {

    // }

    // &str[..]
    str.split_whitespace().next().unwrap()
}

fn second_word(str: &str) -> &str {
    // let i: Vec<&str> = str.split_whitespace().collect();
    // println!("{}", i[1]);
    let bytes = str.as_bytes();
    let mut c = 0;
    let mut start = 0;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if c == 0 {
                c += 1;
                start = i+ 1;
                continue;
            }
            if c == 1 {
                return &str[start..i];
            }
        }
        if c == 1 && i == bytes.len() - 1 {
            return &str[start..];
        }
    }

    &str[..]
}
