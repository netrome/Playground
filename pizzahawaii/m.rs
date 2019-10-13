use std::collections::BTreeMap;

use std::io::{self, Read};

fn generate_test_cases(buffer: &str) -> Vec<Vec<&str>>{
    let mut it = buffer.split_terminator('\n');
    let num_test_cases: usize = it.next().unwrap().parse().unwrap();

    let mut test_cases: Vec<Vec<&str>> = Vec::with_capacity(num_test_cases);

    loop{
        let num_pizzas: usize = match it.next(){
            Some(v) => v.trim().parse().unwrap(),
            None => break,
        };

        test_cases.push(it.by_ref().take(num_pizzas * 3).collect());
    }

    test_cases
}

//fn solve_test_case<'a>(test_case: &[&'a str]) -> Vec<(&'a str, &'a str)>{
fn solve_test_case(test_case: &[&str]) -> String{
    let mut w1_map = BTreeMap::new();
    let mut w2_map = BTreeMap::new();

    //println!("Test case: {:?}", test_case);

    let (contents, pizzas): (Vec<&str>, Vec<&str>) = test_case.iter().partition(|&s| s.chars().nth(0).unwrap().is_numeric());

    let len: usize = pizzas.len() as usize;

    for i in 0..len{
        let val: u64 = 2_u64.pow(i as u32);

        let w1 = contents[i*2];
        let w2 = contents[i*2+1];

        w1.split_whitespace().skip(1).for_each(|i| *w1_map.entry(i).or_insert(0) += val);
        w2.split_whitespace().skip(1).for_each(|i| *w2_map.entry(i).or_insert(0) += val);
    }

    //println!("Pizzas: {:?}", pizzas);
    //println!("w1_map: {:?}", w1_map);
    //println!("w2_map: {:?}", w2_map);

    let w1_inv = w1_map.iter().fold(BTreeMap::new(), |mut acc, (&k, &v)| {
        acc.entry(v).or_insert(Vec::new()).push(k);
        acc
    });

    let w2_inv = w2_map.iter().fold(BTreeMap::new(), |mut acc, (&k, &v)| {
        acc.entry(v).or_insert(Vec::new()).push(k);
        acc
    });
    //println!("w1_inv: {:?}", w1_inv);
    //println!("w2_inv: {:?}", w2_inv);

    let mut ans: Vec<(&str, &str)> = w1_inv.iter().flat_map(|(k, vals)| {
        let mut out = Vec::new();

        if let Some(other) = w2_inv.get(k){
            for &val in vals{
                for &val2 in other{
                    out.push((val, val2));
                }
            }
        }
        out
    }).collect();

    ans.sort();
    //println!("Ans: {:?}", ans);

    let output: String = ans.iter().map(|(a, b)| format!("({}, {})\n", a, b)).collect();

    output
}

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let test_cases = generate_test_cases(&buffer);

    //solve_test_case(&test_cases[0]);
    //println!{"Test cases: {:?}", test_cases};

    let answer: String = test_cases
        .iter()
        .map(|test_case| format!("{}\n", solve_test_case(test_case)))
        .collect();
    
    println!("{}", answer.trim());
    Ok(())
}
