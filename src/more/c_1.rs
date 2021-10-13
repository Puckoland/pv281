fn main() {
    boxes();
    refferences();
    char_count();   
}

fn boxes() {
    let i = Box::new(5);
    let o = *i + 1;
    println!("{} {}", i, o);
}

fn refferences() {
    let a = 10;
    let b = &a;
    let c = &a;
    print!("{} ", b + c);
    let mut d = &a;
    d = &d;
    println!("{}", d + 1);
}

fn char_count() {
    let pangram: &'static str = "aaabbaaccdee";
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    let mut ch: char = chars[0];
    let mut s: String = String::from(ch);
    let mut i: i32 = 0;
    for c in chars {
        if ch != c {
            s = format!("{}{}", s, i);
            s.push(c);
            ch = c;
            i = 0;
        }
        i += 1;
    }
    s = format!("{}{}", s, i);
    println!("{}", s);
}
