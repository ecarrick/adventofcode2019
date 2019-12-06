// range 272091-815432
use fancy_regex::Regex;


fn possible_password(s: String) -> bool {
    const RADIX: u32 = 10;

    let mut haspair = false;
    let mut ispaired = false;
    let mut last_c = Some(' ');

    if s.len() < 1 { return false }

    for i in 1..s.len() {
        let c1 = s.chars().nth(i-1);
        let c2 = s.chars().nth(i);
        if c1.unwrap().to_digit(RADIX).unwrap() > c2.unwrap().to_digit(RADIX).unwrap() { return false }

    }
    let re = Regex::new(r"(?:^|(.)(?!\1))(\d)\2(?!\2)").unwrap();
    return re.is_match(&s).unwrap();
}

fn main() {
    let mut possibles = 0;
    for p in 272091..815432 {
        // let p = 667888;
        let ps = p.to_string();
        if possible_password(ps) {/*println!("{}",p);*/ possibles += 1}
        

    }
    println!("{:?}", possibles);

}
