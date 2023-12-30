use sysinfo::{ System };

const N: u32 = 40;

fn main() {
    let sys = System::new_all();
    let timer = std::time::Instant::now();

    let mut counter = 0;
    loop {
        if counter == N {
            break;
        }

        println!("{}: {}", counter, fibonacci(counter));
        counter += 1;
    }

    println!("Elapsed time: {}ms", timer.elapsed().as_millis());
    println!("Memory usage: {} MB", sys.used_memory() / 1024 / 1024);
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } 

    return fibonacci(n - 1) + fibonacci(n - 2);
}