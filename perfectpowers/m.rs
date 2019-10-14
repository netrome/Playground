use std::io::Read;

fn max_pow(x: i32) -> String{
    let (step, pos) = match x {
        _ if x < 0 => (2, -x),
        _ => (1, x),
    };
    format!("{}\n", (1..33).step_by(step).filter(|pow| (pos as f64).powf(1. / (*pow as f64)).round().powi(*pow) as i32 == pos).last().unwrap())
}

fn main(){
    let mut buff = String::new();
    std::io::stdin().read_to_string(&mut buff).unwrap();

    let ans: String = buff
        .split_terminator('\n')
        .take_while(|s| s.chars().nth(0).unwrap() != '0')
        .map(|i| i.parse().expect("Can't parse stuff"))
        .map(|i| max_pow(i))
        .collect();

    print!("{}", ans);
}
