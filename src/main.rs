use gxhash::{gxhash128, gxhash64};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter::repeat;
use uuid::Uuid;

// limit the length of a string to 20 chars
// the result string is shorter than 20 chars, when the input string is shorter than 20 chars
fn limit_variable_string_length_v1(mut value: String) -> String {
    value.truncate(20);
    value
}

// limit the length of a string to 20 chars
// the result string is always 20 chars long
fn limit_variable_string_length_v2(value: String) -> String {
    value.chars().chain(repeat('1')).take(20).collect()
}

fn hash_uid(uid: Uuid) -> String {
    let mut hasher = DefaultHasher::new();
    uid.hash(&mut hasher);
    hasher.finish().to_string()
}

/// gx_hash_uid is used to generate a unique user id
/// replaces DefaultHasher with https://github.com/ogxd/gxhash
/// Uuid is a generated unique id, e.g. c4d33c4e-74ac-4952-ba43-d454be176bd6
/// the function returns a string with a fixed length of 20 chars, e.g. 13341702002454322048
fn gx_hash_uid64(uid: Uuid) -> String {
    // convert Uuid to bytes
    let bytes: &[u8] = uid.as_bytes();
    // use gxhash64 to hash, this produces a 19 or 20 chars long string
    gxhash64(bytes, 1234).to_string()
}

fn gx_hash_uid128(uid: Uuid) -> String {
    // convert Uuid to bytes
    let bytes: &[u8] = uid.as_bytes();
    // use gxhash128 to hash, this produces a 39 or 40 chars long string
    gxhash128(bytes, 1234).to_string()
}

fn main() {
    println!("Testing gxhash...");
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        gx_hash_uid128, gx_hash_uid64, hash_uid, limit_variable_string_length_v1,
        limit_variable_string_length_v2,
    };
    use uuid::Uuid;

    #[test]
    fn test_hash_uid() {
        let uid = Uuid::new_v4();
        let mut hashed_uid = "".to_string();

        // 1st test with default hasher
        let start_time = Instant::now();
        for _ in 1..10000000 {
            hashed_uid = hash_uid(uid);
        }
        let duration_hash = start_time.elapsed();

        println!(
            "uuid: {}, hashed_uid: {}, len: {}, hash took: {:?} ms",
            uid,
            hashed_uid,
            hashed_uid.len(),
            duration_hash.as_millis()
        );

        // 2nd test with gx_hash_uid64 and string truncate
        let start_time = Instant::now();
        let mut gx_hashed64_uid = "".to_string();
        for _ in 1..10000000 {
            gx_hashed64_uid = gx_hash_uid64(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..10000000 {
            gx_hashed64_uid = limit_variable_string_length_v1(gx_hashed64_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, gx_hashed64_uid v1 (truncate): {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            gx_hashed64_uid,
            gx_hashed64_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        // 3rd test with gx_hash_uid64 and string fill_iter
        let start_time = Instant::now();
        let mut gx_hashed64_uid = "".to_string();
        for _ in 1..10000000 {
            gx_hashed64_uid = gx_hash_uid64(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..10000000 {
            gx_hashed64_uid = limit_variable_string_length_v2(gx_hashed64_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, gx_hashed64_uid v2 (fill_iter): {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            gx_hashed64_uid,
            gx_hashed64_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        // 4th test with gx_hash_uid128 and string truncate
        // this was the fastest test
        let start_time = Instant::now();
        let mut gx_hashed128_uid = "".to_string();
        for _ in 1..10000000 {
            gx_hashed128_uid = gx_hash_uid128(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..10000000 {
            gx_hashed128_uid = limit_variable_string_length_v1(gx_hashed128_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, gx_hash_uid128 v1 (truncate): {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            gx_hashed128_uid,
            gx_hashed128_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        // 5th test with gx_hash_uid128 and string fill_iter
        let start_time = Instant::now();
        let mut gx_hashed128_uid = "".to_string();
        for _ in 1..10000000 {
            gx_hashed128_uid = gx_hash_uid128(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..10000000 {
            gx_hashed128_uid = limit_variable_string_length_v2(gx_hashed128_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, gx_hash_uid128 v2 (fill_iter): {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            gx_hashed128_uid,
            gx_hashed128_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        assert_eq!(gx_hashed64_uid.len(), 20);
        assert_eq!(gx_hashed128_uid.len(), 20);
        // this assert may break, because we have no guarantee that the result string is always 20 chars long
        assert_eq!(hashed_uid.len(), 20);
    }
}
