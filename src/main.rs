#![allow(dead_code)]

extern crate base64;
extern crate hex;


fn main() {
    println!("Hello, cryptopals!");
}
 
//
// Set 1, Challenge 1
//
 
/// Convert a hex string to a base64'd string
fn hex_b64(hex_str: &str) -> String {
    base64::encode(&hex::decode(hex_str).expect("invalid hex string"))
}


#[test]
fn chal1_1() {
    assert_eq!(
        hex_b64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        )
}

//
// Set 1, Challenge 2
//

/// Xor the two iterables together
fn fixed_xor<'a, I>(iter1: I, iter2: I) -> Result<String, String> 
where 
    I: Iterator<Item = &'a u8>,
{
    let bytes: Vec<u8> = iter1.zip(iter2).map(|(a, b)| a ^ b).collect();
    Ok(hex::encode(bytes))
}

#[test]
fn chal1_2() {
    assert_eq!(
        fixed_xor(
            hex::decode("1c0111001f010100061a024b53535009181c").unwrap().iter(),
            hex::decode("686974207468652062756c6c277320657965").unwrap().iter(),
            ).ok(),
        Some("746865206b696420646f6e277420706c6179".to_string()),
        );
}
