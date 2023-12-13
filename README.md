# gxhashtest
compare gxhash with DefaultHasher in Rust

Is using https://github.com/ogxd/gxhash

Run with:

```
cargo test -- --nocapture
```

Example output:

```
uuid: 904ebd40-74e9-4e23-84ea-517501e7abdb, hashed_uid: 14441696786189459178, len: 20, hash took: 288 ms, total took: 674 ms
uuid: 904ebd40-74e9-4e23-84ea-517501e7abdb, gx_hashed_uid: 3782867463144427854, len: 20, hash took: 293 ms, total took: 676 ms
```

The program creates a uuid and has 3 helper functions:
1. hash_uuid to hash a uuid with DefaultHasher
2. gxhash_uuid to hash a uuid with gxhash
3. a string limit function to limit the string to a fixed length of 20 characters

The programm calls the both hash and limit function 1 million times to measure the time spent.

The first "took ms" number is the time spent for the hashing function and the total time includes the limitation to 20 chars.

The goal of the function is, that we get a fixed length shorter uuid (similar what Google does).


