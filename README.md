# gxhashtest

compare gxhash with DefaultHasher in Rust

We want to test here GxHash (a non-cryptographic hashing algorithm) from Olivier Giniaux (published in September 2023) [1].
Also we want to find out, if string iter or string truncate is faster for our use case.

Run with:

```
cargo test --release -- --nocapture
```

(Note: use --release to have faster results)

Example output:

```
running 1 test
uuid: f6c45ae9-4486-4038-ab8e-817c5ca11059, hashed_uid: 836249621649601198, len: 18, hash took: 483 ms
uuid: f6c45ae9-4486-4038-ab8e-817c5ca11059, gx_hashed64_uid v1 (truncate): 14770598964675748883, len: 20, hash took: 368 ms, total took: 400 ms
uuid: f6c45ae9-4486-4038-ab8e-817c5ca11059, gx_hashed64_uid v2 (fill_iter): 14770598964675748883, len: 20, hash took: 369 ms, total took: 809 ms
uuid: f6c45ae9-4486-4038-ab8e-817c5ca11059, gx_hash_uid128 v1 (truncate): 30496146084139209394, len: 20, hash took: 583 ms, total took: 615 ms
uuid: f6c45ae9-4486-4038-ab8e-817c5ca11059, gx_hash_uid128 v2 (fill_iter): 30496146084139209394, len: 20, hash took: 577 ms, total took: 1015 ms
```

The program creates a uuid and has 5 helper functions:

1. hash_uuid to hash a uuid with DefaultHasher
2. gxhash_uuid64 to hash a uuid with gxhash64
3. gxhash_uuid128 to hash a uuid with gxhash128
4. a string iter/fill function to read a string and fill it with '1' chars at the end, if the string is shorter than 20 chars
5. a string truncate function to limit the string to a fixed length of 20 characters

The program calls both hash and limit function 1 billion times to measure the time spent.

The first "took ms" number is the time spent for the hashing function and the total time includes the limitation to 20 chars.

The goal of the program is, that we get shorter uuids with numbers only and a fixed length  (similar what Google does).

[1] <https://github.com/ogxd/gxhash>

[2] <https://www.reddit.com/r/rust/comments/17xgn0t/gxhash_a_new_extremely_fast_and_robust_hashing/?rdt=51749>
