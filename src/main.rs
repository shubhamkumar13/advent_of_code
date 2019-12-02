use std::fs;

fn main() {
    let filename = "input.txt".to_string();
    let contents = fs::read_to_string(filename)
                    .expect("Somthings wrong");
    
    let contents_iter = contents.split('\n');

    let contents_vec = contents_iter
                            .filter(|x| x.parse::<u64>().is_ok())
                            .map(|x| x.parse::<f64>().unwrap())
                            .map(get_fuel)
                            .fold(0, |acc, x| acc + x);
    
    println!("{}", contents_vec);
}

fn get_fuel(x: f64) -> i64 {
    
    let x = (x / 3.0).floor() as i64;
    let x = x - 2;
    if (x as i64) > 0 {
            x + get_fuel(x as f64)
    } else {
        0
    }
}