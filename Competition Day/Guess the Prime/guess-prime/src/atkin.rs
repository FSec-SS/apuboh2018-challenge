pub struct Sieve {
    pub low: usize,
    pub high: usize,
    pub count: usize,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve {
            low: 2,
            high: 1_000,
            count: 2,
        }
    }
}

impl Default for Sieve {
    fn default() -> Self {
        Self {
            low: 2,
            high: 1_000,
            count: 2,
        }
    }
}

impl Iterator for Sieve {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        Some(self.count)
    }
}

pub fn sieve(amount: usize) -> Vec<usize> {
    match amount {
        0 => vec![],
        1 => vec![2],
        2 => vec![2, 3],
        3 => vec![2, 3, 5],
        _ => {
            let mut primes: Vec<usize> = vec![2, 3, 5];
            // Potential primes
            let xs = (2usize..).filter(by_modulo_sixty);

            for x in xs {
                // step 7: skip if x is multiple of any prime
                if has_multiple(&primes, x) {
                    continue;
                };

                // Collect prime
                primes.push(x);

                if primes.len() == amount {
                    break;
                }
            }

            primes
        }
    }
}

/// Atkin's algorithm sieves by applying modulo-sixty to a positive integer.
/// The reminder `r` determines what case should be applied, first, second or
/// third quadratic or it can be flagged as non prime.
fn by_modulo_sixty(n: &usize) -> bool {
    match n % 60 {
        1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => first_quadratic(*n),
        7 | 19 | 31 | 43 => second_quadratic(*n),
        11 | 23 | 47 | 59 => third_quadratic(*n),
        _ => false,
    }
}

/// First quadratic: `4x^2 + y^2 = n`
///
/// Given `x` and `y` must be positive integers, `x` will take values of
/// `x <= sqrt((n - 1) / 4)`.
///
/// `n` will be a potential prime if there are an even amount of solutions.
/// And a solution is represented by any `y` with no reminder (i.e. integer
/// solutions).
fn first_quadratic(n: usize) -> bool {
    let limit = (((n - 1) / 4) as f64).sqrt() as usize + 1;

    (1..limit).fold(0, |acc, x| {
        let y = ((n - 4 * x.pow(2)) as f64).sqrt();
        if y % 1f64 == 0f64 {
            acc + 1
        } else {
            acc
        }
    }) % 2 != 0
}

/// Second quadratic: `3x^2 + y^2 = n`
///
/// Given `x` and `y` must be positive integers, `x` will take values of
/// `x <= sqrt((n - 1) / 3)`.
///
/// `n` will be a potential prime if there are an even amount of solutions.
/// And a solution is represented by any `y` with no reminder (i.e. integer
/// solutions).
fn second_quadratic(n: usize) -> bool {
    let limit = (((n - 1) / 3) as f64).sqrt() as usize + 1;

    (1..limit).fold(0, |acc, x| {
        let y = ((n - 3 * x.pow(2)) as f64).sqrt();
        if y % 1f64 == 0f64 {
            acc + 1
        } else {
            acc
        }
    }) % 2 != 0
}

/// Second quadratic: `3x^2 - y^2 = n` when `x > y`
///
/// Given `x` and `y` must be positive integers, `x` will take values of
/// `x <= sqrt((3 + 2n) / 4) - 1/2`.
///
/// `n` will be a potential prime if there are an even amount of solutions.
/// And a solution is represented by any `y` with no reminder (i.e. integer
/// solutions).
fn third_quadratic(n: usize) -> bool {
    let limit = ((((3 + 2 * n) as f64) / 4.).sqrt() - 0.5) as usize + 1;

    (1..limit).fold(0, |acc, x| {
        let z = (3 * x.pow(2)) as f64 - (n as f64);

        if z < 0f64 {
            return acc;
        }

        let y = (z).sqrt();
        if y % 1f64 == 0f64 {
            acc + 1
        } else {
            acc
        }
    }) % 2 != 0
}

fn has_multiple(primes: &[usize], n: usize) -> bool {
    primes.iter().any(|&p| n % p == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_prime() {
        let actual = sieve(1);
        assert_eq!(&actual, &[2]);
    }

    #[test]
    fn two_prime() {
        let actual = sieve(2);
        assert_eq!(&actual, &[2, 3]);
    }

    #[test]
    fn three_primes() {
        let actual = sieve(3);
        assert_eq!(&actual, &[2, 3, 5]);
    }

    #[test]
    fn twenty_primes() {
        let actual = sieve(20);

        assert_eq!(
            &actual,
            &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]
        );
    }

    #[test]
    fn forty_primes() {
        let actual = sieve(40);

        assert_eq!(
            &actual,
            &vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173,
            ]
        );
    }

    #[test]
    fn thousandth_prime() {
        let actual = sieve(1000);

        assert_eq!(*actual.last().unwrap(), 7919);
    }

    #[test]
    fn twenty_thousandth_prime() {
        let actual = sieve(20_000);

        assert_eq!(*actual.last().unwrap(), 224_737);
    }

    #[test]
    fn modulo_sixty() {
        let pairs = [
            (7, true),
            (41, true),
            (49, false),
            (125, false),
            (169, true),
            (7919, true),
        ];

        for &(n, expected) in &pairs {
            let actual = by_modulo_sixty(&n);

            assert_eq!(actual, expected);
        }
    }
}
