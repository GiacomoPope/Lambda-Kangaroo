# Lambda-Kangaroo
> A Lambda Method for Catching Kangaroos

Trying to implement this in both rust and python. Currently the python is running faster than the rust, which seems off. Maybe someone more familiar with rust can help me out...

- For python, I am using `gmpy2` and computing my big ints as `mpz` types. 
- For Rust, I am using `rug` for my big integers.

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
