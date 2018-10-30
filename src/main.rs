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
fn fixed_xor<'a, I>(iter1: I, iter2: I) -> String
where I: Iterator<Item = &'a u8>,
{
    let bytes: Vec<u8> = iter1.zip(iter2).map(|(a, b)| a ^ b).collect();
    hex::encode(bytes)
}

#[test]
fn chal1_2() {
    assert_eq!(
        fixed_xor(
            hex::decode("1c0111001f010100061a024b53535009181c").unwrap().iter(),
            hex::decode("686974207468652062756c6c277320657965").unwrap().iter(),
            ),
        "746865206b696420646f6e277420706c6179".to_string(),
        );
}


//
// Set 1, Challege 3
//

struct Candidate {
    msg: String,
    key: char,
    score: u32,
}

/// Determine the best candidate decodings of the secret message.
///
fn find_best_candidates(n: usize) -> Vec<Candidate> {
    let msg = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let msg_bytes = &hex::decode(msg).unwrap();

    let mut vals: Vec<Candidate> = (0..255).map(|i| {
        let (decoded_msg, score) = xor_cipher_with_score(msg_bytes.iter(), &i);
        Candidate { msg: decoded_msg, score: score, key: i as char }
    }).collect();

    vals.sort_by_key(|v| v.score);
    vals.into_iter().rev().take(n).collect()
}

/// Xor an iterable with some key and return the result, along with a score
/// indicating its likelihood of being English.
///
fn xor_cipher_with_score<'a, I>(iterable: I, key: &u8) -> (String, u32)
where I: Iterator<Item = &'a u8>
{
    let chars = iterable.map(|x| *x ^ *key).collect();
    let out_str = match String::from_utf8(chars) {
        Err(_) => return ("[not valid unicode]".to_string(), 0),
        Ok(str) => str,
    };

    let common_chars = "AEIOUaeiou ";

    // Count the number of characters likely to be English ASCII.
    let char_count: u32 = out_str.as_bytes().iter().map(|x| {
        if (*common_chars).contains(*x as char) { 1 } else { 0 }
    }).sum();

    (out_str, char_count)
}

#[test]
fn chal1_3() {
    let best = &find_best_candidates(1)[0];

    assert_eq!(best.msg, "Cooking MC's like a pound of bacon");
    assert_eq!(best.key, 'X');
    assert_eq!(best.score, 17);
}
