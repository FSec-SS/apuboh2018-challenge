extern crate guess_prime;
extern crate rand;

use guess_prime::atkin;
use rand::{thread_rng, Rng};
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

/// Choose two random prime numbers from the pre-allocated list of prime numbers. Return two of the
/// prime number and their product.
fn rand_prime(primes: &[usize]) -> (usize, usize, usize) {
    let mut rng = thread_rng();

    let x = rng.choose(primes).unwrap();
    let y = rng.choose(primes).unwrap();

    let product = x * y;
    (*x, *y, product)
}

/// Handle connection by looping around flag. Send a character for each prime number guess. The
/// real character if valid or else `x`. Keep guessing even if flag is exhausted.
fn handle_connection(mut stream: TcpStream, primes: &Arc<Vec<usize>>) -> std::io::Result<()> {
    let mut flag = "rustaceans@apuboh2018".bytes();
    // Use BufReader for Lines
    let mut lines = BufReader::new(stream.try_clone()?).lines();
    loop {
        let (x, y, product) = rand_prime(&primes);

        writeln!(stream, "{}", product)?;
        println!("{} * {} == {}", x, y, product);

        // Convert None into Err(io::Error) to propogate error
        let line = lines.next();
        let line = line.unwrap_or_else(|| Err(Error::new(ErrorKind::Other, "Client quit")));

        // Consume the flag iterator on each loop
        let flag = flag.next();

        // Set return character to flag if one prime number matches, use b'x' if iterator is done
        let c = match line?.parse::<usize>() {
            Ok(n) if n == x || n == y => flag.unwrap_or(b'x'),
            _ => b'x',
        };

        writeln!(stream, "{}", c as char)?;
    }
}

fn main() {
    let primes: Vec<_> = atkin::sieve(10_000)
        .into_iter()
        .filter(|&v| v > 10_000 as usize)
        .collect();
    let primes = Arc::new(primes);

    let listener = TcpListener::bind("0.0.0.0:7000").unwrap();
    println!("server running on 0.0.0.0:7000");

    for stream in listener.incoming() {
        let primes = Arc::clone(&primes);

        thread::spawn(move || {
            let stream = stream.unwrap();
            let addr = stream.peer_addr().unwrap();

            println!("> {}", addr);

            // Result must be Err, use ok to remove the warning
            handle_connection(stream, &primes).ok();

            println!("< {}", addr);
        });
    }
}
