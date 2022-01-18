use std::io;
fn main() {
    let mut input = String::new();
    println!("Starting number");
    io::stdin().read_line(&mut input).unwrap();
    let start: i64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("End number");
    io::stdin().read_line(&mut input).unwrap();
    let end: i64 = input.trim().parse().unwrap();
        
    let mut found: i64 = 0;

    for count in start..end {
        if count % 2 != 0 {        
            // If prime check passes, increment found and print count.
            if check_prime(&count) { 
                found += 1;
                println!("{}",&count);
            }
        }
    }

    println!("{}: {}","Prime numbers found",&found);

    fn check_prime(count: &i64) -> bool { 
        // Find stopping number to loop to
        let mut stop = ((*count as f64).sqrt() + 1.0) as i64;
        // Start at 3. Counts until stop value.
        for i in 3..stop {       
            // Check if i is even
            if i % 2 != 0 {      
                // Check if count is divisable by i
                if count % i == 0 { 
                    return false
                }
            }
        }
        return true
    }
}