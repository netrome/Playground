use std::io::Read;
use std::collections::BTreeMap;


fn mean(v: &[u64]) -> f64{
    let sum: u64 = v.iter().sum();
    sum as f64 / v.len() as f64
}


fn median(v: &[u64]) -> f64{
    let l = (v.len() as f64 - 1.)  / 2.;
    (v[l.ceil() as usize] + v[l.floor() as usize]) as f64 / 2.
}


fn mode(v: &[u64]) -> u64{
    let counts = v.iter().fold(BTreeMap::<u64, u64>::new(), |mut map, element| {
        *map.entry(*element).or_insert(0) += 1;
        map
    });

    let mut all: Vec<(&u64, &u64)> = counts.iter().collect();
    all.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    return *all[0].0
}


fn std(v: &[u64]) -> f64{
    let squared: Vec<u64> = v.iter().map(|i| i.pow(2)).collect();
    return (mean(&squared) - mean(v).powf(2.)).sqrt()
}


fn conf(v: &[u64]) -> (f64, f64){
    let dev = 1.96 * std(v) / (v.len() as f64).sqrt();
    let mean = mean(v);
    return (mean - dev, mean + dev)
}


fn main(){
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut numbers: Vec<u64> = buffer.split('\n').nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
    numbers.sort();

    println!("{:.*}", 1, mean(&numbers));
    println!("{:.*}", 1, median(&numbers));
    println!("{}", mode(&numbers));
    println!("{:.*}", 1, std(&numbers));

    let (c1, c2) = conf(&numbers);
    println!("{:.*} {:.*}", 1, c1, 1, c2);
}
