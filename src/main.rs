use gxhash::gxhash64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

fn limit_variable_string_length(value: String) -> String {
    let mut array_tmp = [0u8; 20];
    array_tmp[..value.len()].copy_from_slice(value.as_bytes());
    // convert the array of bytes to a string and return it
    array_tmp.iter().map(|&i| char::from(i)).collect::<String>().replace('\x00', "1")
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
fn gx_hash_uid(uid: Uuid) -> String {
    // convert Uuid to bytes
    let bytes: &[u8] = uid.as_bytes();
    // use gxhash64 to hash, this produces a 19 or 20 chars long string
    gxhash64(bytes, 1234).to_string()
}

fn main() {
    println!("Testing gxhash...");
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{gx_hash_uid, hash_uid, limit_variable_string_length};
    use uuid::Uuid;

    #[test]
    fn test_hash_uid() {
        let uid = Uuid::new_v4();
        let mut hashed_uid = "".to_string();

        let start_time = Instant::now();
        for _ in 1..1000000 {
            hashed_uid = hash_uid(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..1000000 {
            hashed_uid = limit_variable_string_length(hashed_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, hashed_uid: {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            hashed_uid,
            hashed_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        let start_time = Instant::now();
        let mut gx_hashed_uid = "".to_string();
        for _ in 1..1000000 {
            gx_hashed_uid = gx_hash_uid(uid);
        }
        let duration_hash = start_time.elapsed();

        for _ in 1..1000000 {
            gx_hashed_uid = limit_variable_string_length(gx_hashed_uid);
        }
        let duration = start_time.elapsed();

        println!(
            "uuid: {}, gx_hashed_uid: {}, len: {}, hash took: {:?} ms, total took: {:?} ms",
            uid,
            gx_hashed_uid,
            gx_hashed_uid.len(),
            duration_hash.as_millis(),
            duration.as_millis()
        );

        assert_eq!(gx_hashed_uid.len(), 20);
        assert_eq!(hashed_uid.len(), 20);
    }
}
