use std::thread;
use std::io;


fn main() {
    let mut input = String::new();
    println!("Start number: ");
    io::stdin().read_line(&mut input).unwrap();
    let start: i64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("End number number: ");
    io::stdin().read_line(&mut input).unwrap();
    let end: i64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("n of threads to use: ");
    io::stdin().read_line(&mut input).unwrap();
    let nthreads: i64 = input.trim().parse().unwrap();

    let range: i64 = (end-start) / nthreads;

    let mut all_primes : Vec<i64> = Vec::new();
    
    let mut threads = Vec::new();

    for i in 0..nthreads {
        // First thread:
        if i == 0 {
            threads.push(thread::spawn(move || {
                return prime_check(start, start+range, i);
            }));
        } else if i == nthreads -1 {
            threads.push(thread::spawn(move || {
                return prime_check(end-range, end, i);
            }));
        } else {
            threads.push(thread::spawn(move || {
                return prime_check(start+(i*range), start+((i+1)*range)+1, i);
            }));
        }
    }

    for thread in threads {
        let mut result = thread.join().unwrap();
        all_primes.append(&mut result);
    }

    println!("{}", "All primes: ");
    println!("{:?}", all_primes);

    fn prime_check(from: i64, to: i64, thr: i64) -> Vec<i64>{
        let mut primes : Vec<i64> = Vec::new();

        for count in from..to {
            if count == 1 {
                continue;
            } else if count == 2 {
                primes.push(count);
            } else if count % 2 != 0 {        
                // If prime check passes, increment found and print count.
                if check_prime(&count) { 
                    primes.push(count);
                    println!("{:} {:} {:}", count, "By thread: ", thr+1);
                }
            }  
        }
        // println!("{:?}",&primes);
        return primes;
    }
    
    fn check_prime(count: &i64) -> bool { 
        // Find stopping number to loop to
        let stop = ((*count as f64).sqrt() + 1.0) as i64;
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
        return true;
    }
}