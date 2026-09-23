// sha256 algo description : https://sha256algorithm.com/
// for hex inputs : https://blockchain-academy.hs-mittweida.de/sha-256-generator/
#![allow(dead_code)]    // To avoid unused functions warnings

// constants
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];
const H: [u32; 8] = [
    0x6a09e667,
    0xbb67ae85,
    0x3c6ef372,
    0xa54ff53a,
    0x510e527f,
    0x9b05688c,
    0x1f83d9ab,
    0x5be0cd19
];

mod byte_encoding {
    // used command "cargo add hex" to import hex dependency to Cargo.toml

    pub fn encode(input: &str) -> Vec<u8> {
        input.as_bytes().to_vec()
    }

    pub fn print_hexa(input: Vec<u8>) {
        for byte in input {
            print!(" {:02x}", byte);
        }
        println!("")
    }

    pub fn print_bin(input: Vec<u8>) {
        for byte in input {
            print!(" {:08b}", byte);
        }
        println!("");
    }

    pub fn my_decode(input: Vec<u8>) -> String {
        String::from_utf8(input).expect("Invalid UTF-8")
    }

    pub fn hex_to_bytes(input: &str) -> Vec<u8> {
        hex::decode(input).expect("Decoding failed")
    }

    pub fn to_hex(input: [u32; 8]) -> String {  //Peux pas retourner un &str car c'est une ref vers var locale (détruite à la fin de la fct)
        let mut res: String  = String::from("");
        for w in input {
            res += &format!("{:08x}", w); // opérateur + fonctionne que String + &str
            // {:08x} -> largeur minimum de 8 caractères et padding avec des 0
        }
        res
        // possible de tout faire en une ligne avec iter mais flemme
    }
}


use std::convert::TryInto;
use crate::byte_encoding::{encode, hex_to_bytes, to_hex};
use std::io::{self};


fn block_padding(mut msg: Vec<u8>) -> Vec<u8> {
    let msg_len: u64 = (msg.len() * 8) as u64;  //Rust ne fait pas de conversion implicite en entiers, donc utilise "as u64"
    
    // Append bit 1
    msg.push(0b10000000);
    
    // Append zeros until length is 512-64 = 448 bits = 56 bytes (mod 64 bytes or 512bits)
    while msg.len()%64 != 56 {  //(msg.len()%64 < 56) || (msg.len()%64 > 56) il me manquait la 2e condition
        msg.push(0);
    }
    
    // Append length of original message as a 64-bit big-endian integer
    msg.extend(msg_len.to_be_bytes());
    // fonctionne car .to_be_bytes retourne le type [u8; taille en u8] et qu'un array est itérable
    
    msg
}


fn msg_schedule(block: Vec<u8>, chunk_nb: usize) -> [u32; 64] {
    let mut scheduler =  [0u32; 64];// il faut toujours initialiser les variables par valeur défaults
    let chunk_start = chunk_nb*64;

    //Copy the chunk_nb-nth 512bits chunk into 1st 16 words of scheduler
    for w in 0..16 {
        // block[w*4 .. w*4+4] renvoie un slice : une vue (pointeur+longueur) sur des élém contigus de mm type (marche aussi pour vec donc)
        // .try_into() : conversion vers Resultat avec type voulu (ici Result<[u8;4], Self::Error>)
        // .unwrap() : Result -> val, panic si mauvaise length
        // u32::from_be_bytes : array -> u32
        scheduler[w] = u32::from_be_bytes(block[(chunk_start + w*4) .. (chunk_start + w*4+4)].try_into().unwrap());
    }
    
    for w in 16..64 {
        let wa = scheduler[w-16];
        let mut wb = scheduler[w-15];
        let wc = scheduler[w-7];
        let mut wd = scheduler[w-2];

        let wb7 = wb.rotate_right(7);
        let wb18 = wb.rotate_right(18);
        let wb3 = wb >> 3; //ne déplace pas le ownership car u32 est un type copiable
        wb = wb7 ^ wb18 ^ wb3;

        let wd17 = wd.rotate_right(17);
        let wd19 = wd.rotate_right(19);
        let wd10 = wd >> 10;
        wd = wd17 ^ wd19 ^ wd10;

        //scheduler[w] = wa + wb + wc + wd; => overflow panic, types int ne forcent pas opérations modulo 2**taille
        scheduler[w] = wa.wrapping_add(wb).wrapping_add(wc).wrapping_add(wd);
    }

    scheduler
}


fn compression(schedule: [u32; 64], init_hash_val: [u32; 8]) -> [u32; 8] {
    let mut work_var = init_hash_val;
    
    for i in 0..64 {
        let sigma1 = work_var[4].rotate_right(6) ^ work_var[4].rotate_right(11) ^ work_var[4].rotate_right(25);
        let choice = (work_var[4] & work_var[5]) ^ ((!work_var[4]) & work_var[6]);
        let sigma0 = work_var[0].rotate_right(2) ^ work_var[0].rotate_right(13) ^ work_var[0].rotate_right(22);
        let majority = (work_var[0] & work_var[1]) ^ (work_var[0] & work_var[2]) ^ (work_var[1] & work_var[2]);
        let temp1 = work_var[7].wrapping_add(sigma1).wrapping_add(choice).wrapping_add(K[i]).wrapping_add(schedule[i]);
        let temp2 = sigma0.wrapping_add(majority);

        work_var[7] = work_var[6];
        work_var[6] = work_var[5];
        work_var[5] = work_var[4];
        work_var[4] = work_var[3].wrapping_add(temp1);
        work_var[3] = work_var[2];
        work_var[2] = work_var[1];
        work_var[1] = work_var[0];
        work_var[0] = temp1.wrapping_add(temp2);
    }

    init_hash_val.iter().zip(work_var.iter()).map(|(h,w)| h.wrapping_add(*w))
                .collect::<Vec<u32>>().try_into().unwrap()
}


// there is no type u256, so we return the result as an array of u32
pub fn sha256(input: &str) -> [u32; 8] {
    let encoded_input = encode(input);
    let padded_block = block_padding(encoded_input);
    let nb_chunk = padded_block.clone().len()/64;
    let mut current_h_val = H;

    for i in 0..nb_chunk {
        let scheduled = msg_schedule(padded_block.clone(), i);
        current_h_val = compression(scheduled, current_h_val);
    }

    current_h_val
}


pub fn sha256_hex(input: &str) -> String {
    // Receives and outputs hexadecimal string
    let input_bytes = hex_to_bytes(input);
    let padded_block = block_padding(input_bytes);
    let nb_chunk = padded_block.clone().len()/64;
    let mut current_h_val = H;

    for i in 0..nb_chunk {
        let scheduled = msg_schedule(padded_block.clone(), i);
        current_h_val = compression(scheduled, current_h_val);
    }

    to_hex(current_h_val)
}

fn main() -> io::Result<()> {
    // std::io doc : https://doc.rust-lang.org/std/io/index.html
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let hashed: String = sha256_hex(input.trim());  // read_line inclut le '\n' à la fin, il faut donc trim

    println!("{}", hashed.trim());  // println! ajoute un '\n' à la fin !!! -> .strip() pour l'enlever dans le python
    Ok(())
}



#[cfg(test)]
mod tests {
    // To run tests, run : cargo test
    mod test_byte_encoding {
        use crate::byte_encoding::*;    // crate = chemin racine
        #[test]
        fn encode_decode() {
            let input = "Hello";
            let encoded: Vec<u8> = encode(input);
            let decoded = my_decode(encoded.clone());
            assert_eq!(input, decoded);
        }

        #[test]
        fn manual_tests() {
            let input = "Hello";
            assert_eq!(encode(input), vec![0b01001000, 0b01100101, 0b01101100, 0b01101100, 0b01101111]);
        }

        #[test]
        fn print_test() {
            // To print output with tests, run : cargo test -- --nocapture
            let input = "Testttt";
            let encoded = encode(input);
            print_bin(encoded.clone());
            print_hexa(encoded);
        }
        #[test]
        fn test_hex_byte() {
            let input = "41736b507974686f6e2e636f6d";
            let input_tab: [u32; 8] = [0xabcd1234, 0xabcd1234, 0xabcd1234, 0xabcd1234, 0xabcd1234, 0xabcd1234, 0xabcd1234, 0xabcd1234];
            
            assert_eq!(hex_to_bytes(input), encode("AskPython.com"));
            assert_eq!(to_hex(input_tab), "abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234");
        }
    }

    mod test_fn_algo {
        use crate::*;   // encode est accessible car dans racine tu as la ligne "use crate::byte_encoding::encode;""

        const INPUT: &str = "hello";
        //let input_encoded = encode(INPUT);    => let n'est pas autorisé en dehors d'une fn

        #[test]
        fn test_padding() {
            let input_encoded = encode(INPUT);
            let mut padded = block_padding(input_encoded);

            assert_eq!(padded[0], 0b01101000);
            assert_eq!(padded[56], 0);
            assert_eq!(padded[63], 0b00101000);

            let input_hex = "2d52447d1244d2ebc28650e7b05654bad35b3a68eedc7f8515306b496d75f3e73385dd1b002625024b81a02f2fd6dffb6e6d561cb7d0bd7a";
            let input_byte = hex_to_bytes(input_hex);
            padded = block_padding(input_byte);
            assert_eq!(padded.len(), 128);
            assert_eq!(padded[127], 0b11000000);
            assert_eq!(padded[126], 0b00000001);
            assert_eq!(padded[0], 0b00101101);
        }
        #[test]
        fn test_scheduler() {
            let input_encoded = encode(INPUT);
            let padded = block_padding(input_encoded);
            let scheduled = msg_schedule(padded, 0);

            assert_eq!(scheduled[48], 0b10100011011111110100101010000001);
            assert_eq!(scheduled[0], 0b01101000011001010110110001101100);
            assert_eq!(scheduled[63], 0b00111110001111001011010101100001)
        }
        #[test]
        fn test_compression() {
            let input_encoded = encode(INPUT);
            let padded = block_padding(input_encoded);
            let scheduled = msg_schedule(padded, 0);
            let new_h_val = compression(scheduled, H);

            assert_eq!(new_h_val[0], 0x2cf24dba);
            assert_eq!(new_h_val[3], 0xc5b9e29e);
            assert_eq!(new_h_val[7], 0x938b9824);
        }
        #[test]
        fn test_sha256() {
            let input = "1. Encode the input to binary using UTF-8 and append a single '1' to it.  2. Prepend that binary to the message block.";
            
            assert_eq!(sha256(input), [0x487af802, 0xa0408c0c, 0x5bef74ec, 0xe60a2370, 0x95496cd3, 0x0d6a0060, 0x8aedefa5, 0x81626083]);
            assert_eq!(sha256(""), [0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855]);
            assert_eq!(sha256("abc"), [0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223,0xb00361a3, 0x96177a9c, 0xb410ff61, 0xf20015ad]);
        }
        #[test]
        fn test_sha_hex() {
            let input = "4d6f6e20696d706c656d2064652073686132353620657374206e69636b656c2021";   //corresponding text : "Mon implem de sha256 est nickel !"
            assert_eq!(sha256_hex(input), String::from("50a8390e377335f42831522121c855d26ff707a846a89517a850450006be0591"));
        }
    }
}