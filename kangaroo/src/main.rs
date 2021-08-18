use rug::{ops::Pow, Integer};
use std::str::FromStr;

fn f(x: &Integer, k: &u32) -> Integer {
    let exp = Integer::from(x % k).to_u32().unwrap();
    Integer::from(2).pow(exp)
}

/// Determines the mean of the pseudorandom
/// function f(x). The mean is multiplied
/// by a small factor, here we choose θ=4
/// which aligns with 0.98 probability of
/// success
///
/// https://www.ams.org/journals/mcom/1978-32-143/S0025-5718-1978-0491431-9/S0025-5718-1978-0491431-9.pdf
/// page 922
fn determine_mean(k: &u32) -> u32 {
    let mut n = Integer::from(0);

    for i in 0..*k {
        let x = Integer::from(i);
        n += f(&x, &k);
    }
    n /= k;
    n *= 4;

    (n).to_u32().unwrap()
}

/// Determines the max step size for the walks
/// Here we use 2 ^ (x % k) and determine the max
/// value of k from the bounds a,b. We then
/// determine the mean of f(x,k).
fn determine_steps(a: &u64, b: &u64) -> (u32, u32) {
    let diff: f64 = (b - a) as f64;
    let sqrt_ba = (diff).sqrt();
    let log_sqrt_ba = (sqrt_ba).log2();
    let log_log_sqrt_ba = (log_sqrt_ba).log2();
    let k = (log_log_sqrt_ba + log_sqrt_ba - 2_f64).ceil() as u32;
    let n = determine_mean(&k);

    (k, n)
}

fn kangaroo(p: Integer, g: Integer, y: Integer, a: u64, b: u64) -> Integer {
    let (k, n) = determine_steps(&a, &b);
    println!("[+] Running with kangaroos with k={:?}, N={:?}", k, n);

    let mut x_tame = Integer::from(0);
    let mut y_tame = g.clone().pow_mod(&Integer::from(b), &p).unwrap();

    for _i in 0..n {
        x_tame += f(&y_tame, &k);

        y_tame *= g.clone().pow_mod(&f(&y_tame, &k), &p).unwrap();
        y_tame %= &p;
    }

    let mut x_wild = Integer::from(0);
    let mut y_wild = y.clone();
    let x_max = Integer::from(b - a) + &x_tame;

    while x_wild < x_max {
        x_wild += f(&y_wild, &k);

        y_wild *= g.clone().pow_mod(&f(&y_wild, &k), &p).unwrap();
        y_wild %= &p;

        if y_wild == y_tame {
            return b + x_tame - x_wild;
        }
    }

    return Integer::from(0);
}

/// Upper bound on x is 2^20
fn challenge_one() -> Integer {
    let p = Integer::from_str("11470374874925275658116663507232161402086650258453896274534991676898999262641581519101074740642369848233294239851519212341844337347119899874391456329785623").unwrap();
    let g = Integer::from_str("622952335333961296978159266084741085889881358738459939978290179936063635566740258555167783009058567397963466103140082647486611657350811560630587013183357").unwrap();
    let y = Integer::from_str("7760073848032689505395005705677365876654629189298052775754597607446617558600394076764814236081991643094239886772481052254010323780165093955236429914607119").unwrap();

    let a: u64 = 0;
    let b: u64 = 2;
    let b = b.pow(20);
    let x = kangaroo(p, g, y, a, b);

    x
}

/// Upper bound on x is 2^40
fn challenge_two() -> Integer {
    let p = Integer::from_str("11470374874925275658116663507232161402086650258453896274534991676898999262641581519101074740642369848233294239851519212341844337347119899874391456329785623").unwrap();
    let g = Integer::from_str("622952335333961296978159266084741085889881358738459939978290179936063635566740258555167783009058567397963466103140082647486611657350811560630587013183357").unwrap();
    let y = Integer::from_str("9388897478013399550694114614498790691034187453089355259602614074132918843899833277397448144245883225611726912025846772975325932794909655215329941809013733").unwrap();

    let a: u64 = 0;
    let b: u64 = 2;
    let b = b.pow(40);
    let x = kangaroo(p, g, y, a, b);

    x
}

/// Upper bound on x is 2^50
fn challenge_three() -> Integer {
    let p = Integer::from_str("11470374874925275658116663507232161402086650258453896274534991676898999262641581519101074740642369848233294239851519212341844337347119899874391456329785623").unwrap();
    let g = Integer::from_str("622952335333961296978159266084741085889881358738459939978290179936063635566740258555167783009058567397963466103140082647486611657350811560630587013183357").unwrap();
    let y = Integer::from_str("2585439547369344827783631096833320879332017623671584623700759360927091364523327695790646754005150031219003137901554140643625863983591040863107524637438003").unwrap();

    let a: u64 = 0;
    let b: u64 = 2;
    let b = b.pow(50);
    let x = kangaroo(p, g, y, a, b);

    x
}

fn main() {
    // Rough timing for challenge 1
    // ./target/release/kangaroo  0.01s user 0.00s system 6% cpu 0.189 total
    let x1 = challenge_one();
    println!("[+] Secret found: x={:?}", x1);

    // Rough timing for challenge 2
    // ./target/release/kangaroo  14.21s user 0.04s system 98% cpu 14.420 total
    let x2 = challenge_two();
    println!("{:?}", x2);

    // Rough timing for challenge 3
    // ./target/release/kangaroo  533.39s user 3.73s system 97% cpu 9:13.47 total
    // let x3 = challenge_three();
    // println!("{:?}", x3);
}

#[test]
fn test_f() {
    // k = 12
    assert_eq!(f(&Integer::from(2), &12), Integer::from(4));
    assert_eq!(f(&Integer::from(22), &12), Integer::from(1024));
    assert_eq!(f(&Integer::from(222), &12), Integer::from(64));
    assert_eq!(f(&Integer::from(2222), &12), Integer::from(4));
    // k = 23
    assert_eq!(f(&Integer::from(2), &23), Integer::from(4));
    assert_eq!(f(&Integer::from(22), &23), Integer::from(4194304));
    assert_eq!(f(&Integer::from(222), &23), Integer::from(32768));
    assert_eq!(f(&Integer::from(2222), &23), Integer::from(16384));
}
