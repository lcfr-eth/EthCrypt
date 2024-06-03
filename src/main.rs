use ecies::{decrypt, encrypt};
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use hex;
use std::{env, fs::File, io::{Read, Write}};

fn test_functionality(private_key_str: &str) {
    const MSG: &str = "helloworld";
    let secret_bytes = hex::decode(private_key_str).expect("Failed to decode hex string");
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&secret_bytes).expect("Failed to create SecretKey");
    let pk = PublicKey::from_secret_key(&secp, &sk);

    let sk_bytes = &sk[..];
    let pk_bytes = &pk.serialize_uncompressed();

    let pk_hex = hex::encode(pk_bytes);
    println!("Public Key in Hex: {}", pk_hex);

    let msg = MSG.as_bytes();
    let encrypted = encrypt(pk_bytes, msg).unwrap();
    println!("encrypted: {:x?}\n", encrypted);

    let enc_hex = hex::encode(&encrypted);
    println!("encrypted: {}", enc_hex);

    let binding = decrypt(sk_bytes, &encrypted).unwrap();
    let dec_str = std::str::from_utf8(&binding).expect("Failed to convert decrypted data to UTF-8 string");
    println!("decrypted: {}\n", dec_str);
}

fn encrypt_message(pub_key_hex: &str, message: &str) {
    let pub_key_bytes = hex::decode(pub_key_hex).expect("Invalid hex for public key");
    
    if pub_key_bytes.len() != 65 {
        panic!("Public key must be 65 bytes long (uncompressed)");
    }
    
    if pub_key_bytes[0] != 0x04 {
        panic!("Public key must be uncompressed and start with 0x04");
    }

    let encrypted = encrypt(&pub_key_bytes, message.as_bytes()).unwrap();

    let encrypted_hex = hex::encode(&encrypted);
    println!("Message encrypted: {}", encrypted_hex);
}

fn decrypt_message(private_key_str: &str, encrypted_hex: &str) {
    let secret_bytes = hex::decode(private_key_str).expect("Failed to decode hex string");
    let sk = SecretKey::from_slice(&secret_bytes).expect("Failed to create SecretKey");

    let sk_bytes = &sk[..];

    let enc_bytes = hex::decode(encrypted_hex).expect("failed to decode hex");

    let binding = decrypt(sk_bytes, &enc_bytes).unwrap();

    let decrypted_message = std::str::from_utf8(&binding).expect("Failed to convert decrypted bytes to string");
    println!("Decrypted message: {}", decrypted_message);
}

fn encrypt_file(pub_key_hex: &str, input_file_path: &str, output_file_path: &str) {
    let pub_key_bytes = hex::decode(pub_key_hex).expect("Invalid hex for public key");

    if pub_key_bytes.len() != 65 {
        panic!("Public key must be 65 bytes long (uncompressed)");
    }

    if pub_key_bytes[0] != 0x04 {
        panic!("Public key must be uncompressed and start with 0x04");
    }

    let mut file = File::open(input_file_path).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let encrypted_data = encrypt(&pub_key_bytes, &data).expect("Encryption failed");

    let mut output_file = File::create(output_file_path).expect("Failed to create output file");
    output_file.write_all(&encrypted_data).expect("Failed to write encrypted data");

    println!("File encrypted successfully and saved to {}", output_file_path);
}

fn decrypt_file(private_key_str: &str, input_file_path: &str, output_file_path: &str) {
    let secret_bytes = hex::decode(private_key_str).expect("Failed to decode hex string");
    let sk = SecretKey::from_slice(&secret_bytes).expect("Failed to create SecretKey");

    let mut file = File::open(input_file_path).expect("Failed to open input file");
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data).expect("Failed to read input file");

    let decrypted_data = decrypt(sk.as_ref(), &encrypted_data).expect("Decryption failed");

    let mut output_file = File::create(output_file_path).expect("Failed to create output file");
    output_file.write_all(&decrypted_data).expect("Failed to write decrypted data");

    println!("Decryption complete. Decrypted data written to: {}", output_file_path);
}

fn get_public_key(private_key_str: &str) {
    let secret_bytes = hex::decode(private_key_str).expect("Failed to decode hex string");
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&secret_bytes).expect("Failed to create SecretKey");
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_bytes = &pk.serialize_uncompressed();
    let pk_hex = hex::encode(pk_bytes);
    println!("Public Key in Hex: {}", pk_hex);
}

fn main() {

    let pkey = env::var("PKey").expect("not valid key");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: --test | --encrypt <public_key_hex> <message> | --decrypt <secret_key_hex> <encrypted_hex>");
        return;
    }

    match args[1].as_str() {
        "--test" => test_functionality(&pkey),
        "--pubkey" => get_public_key(&pkey),
        "--encrypt" => {
            if args.len() != 4 {
                println!("Usage: --encrypt <public_key_hex> <message>");
            } else {
		        let pubkey = &args[2];
                let message = &args[3];
                encrypt_message(pubkey, message);
            }
        },
        "--decrypt" => {
            if args.len() != 3 {
                println!("Usage: --decrypt <encrypted_hex>");
            } else {
                let encrypted_hex = &args[2];
                decrypt_message(&pkey, encrypted_hex);
            }
        },
        "--encrypt-file" => {
            if args.len() != 5 {
                println!("Usage: --encrypt-file <public_key_hex> <input_file>");
            } else {
                let pubkey = &args[2];
                let file_path = &args[3];
		        let out_path = &args[4];
                encrypt_file(pubkey, file_path, out_path);
            }
        },
        "--decrypt-file" => {
            if args.len() != 4 {
                println!("Usage: --decrypt-file <input_file> <output_file>");
            } else {
                let file_path = &args[2];
		        let out_path = &args[3];
                decrypt_file(&pkey, file_path, out_path);
            }
        },
        _ => println!("Invalid option. Use --test, --encrypt, or --decrypt."),
    }
}