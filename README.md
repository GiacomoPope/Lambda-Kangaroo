# Lambda-Kangaroo
> A Lambda Method for Catching Kangaroos

## Disclaimer

I am VERY new to rust. I'm trying to do some fun crypto things as a way to learn, but if you're a rust fan, please accept my apologies for the state of my `.rs` files... 

## Algorithm 

```
// Full discussion 
// TODO
```

This implementation follows:

- [Pollard's original paper](https://www.ams.org/journals/mcom/1978-32-143/S0025-5718-1978-0491431-9/S0025-5718-1978-0491431-9.pdf)
- [CryptoPals 58](https://toadstyle.org/cryptopals/58.txt)

I have implemented this in both python3 and rust. I'm more familiar with python3, so used this as a way to make sure the algorithm ran as expected then did my best to make a reasonable rust implementation of the same algorithm. Currently the python is running at approximately the same speed as rust, which seems off.

Maybe someone more familiar with rust can help me out...

- For python, I am using `gmpy2` and computing my big ints as `mpz` types. 
- For Rust, I am using `rug` for my big integers.

As underneath the good this is all GMP, maybe the similar run speed is to be expected... Although I would still expect rust to outperform python.

## Upper bound 2^40

```
Jack: python % time python3 kangaroo.py
python3 kangaroo.py  13.85s user 0.06s system 99% cpu 14.022 total

Jack: rust % time ./target/release/kangaroo
./target/release/kangaroo  14.18s user 0.05s system 99% cpu 14.313 total
```

## Upper bound 2^50

```
Jack: python % time python3 kangaroo.py
python3 kangaroo.py  491.32s user 1.58s system 99% cpu 8:16.86 total

Jack: rust % time ./target/release/kangaroo
./target/release/kangaroo  533.39s user 3.73s system 97% cpu 9:13.47 total
```
