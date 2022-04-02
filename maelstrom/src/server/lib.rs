use rand::Rng;

const CHARSET: &[u8] = b"0123456789abcdef";
// generate a random hexadecimal string of length 16
pub fn generate_uid() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut uid: Vec<u8> = Vec::with_capacity(16);
    for _ in 0..16 {
        uid.push(CHARSET[rng.gen_range(0..16)]);
    }
    return uid;
}
