use std::thread;

fn main() {
    // let start: f64 = 0;
    let end: i64 = 1000;
    let nthreads: i64 = 20;
    let range: i64 = end / nthreads;

    let mut all_primes : Vec<i64> = Vec::new();
    
    let mut threads = Vec::new();

    for i in 0..nthreads {
        if i == 0 {
            threads.push(thread::spawn(move || {
                return prime_check(1 + (i*range), (i + 1)*range)
            }));
        } else {
            if i == nthreads -1 {
                threads.push(thread::spawn(move || {
                    return prime_check(i*range, end)
                }));
            } else {
                threads.push(thread::spawn(move || {
                    return prime_check(i*range, (i+1)*range)
                }));
            }
        }
    }

    for thread in threads {
        let result = thread.join().unwrap();
        all_primes.extend(result);
    }

    println!("{:?}", all_primes);

    fn prime_check(from: i64, to: i64) -> Vec<i64>{
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