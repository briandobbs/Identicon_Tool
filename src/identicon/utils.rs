use sha2::{Sha256, Digest};

pub fn hash_input(input: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // take input test for hashing
    hasher.update(input);

    // get the hashed string by formatting result of hasher finalize
    let result: String = format!("{:X}", hasher.finalize());

    return result;
}

pub fn is_even(input: u8) -> bool {

    if let 0 = input % 2 {
        // result is even
        return true;
    } else {
        // result is odd
        return false;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_input() {
        let result: String = hash_input("Hello World!".to_string());
        assert_eq!(result, "7F83B1657FF1FC53B92DC18148A1D65DFC2D4B1FA3D677284ADDD200126D9069");
    }

}