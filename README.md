# EthCrypt  

Use [ECIES](https://medium.com/asecuritysite-when-bob-met-alice/elliptic-curve-integrated-encryption-scheme-ecies-encrypting-using-elliptic-curves-dc8d0b87eaa) to encrypt text and files <= 1G using an ethereum public key. 

Decrypt using corresponding private key.

## Usage
Requires the ```Public Key``` of an Ethereum address to encrypt the data with.

```% export PKey=0000000000000000000000000000000000000000000000000000000000000002```

```
% cargo run
Encrypt/Decrypt text & files using ETH keypairs

Usage: EthCrypt <COMMAND>

Commands:
  test          
  pubkey        
  encrypt       
  decrypt       
  encrypt-file  
  decrypt-file  
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Test 
```
% cargo run test                                                              

Public Key in Hex: 04c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee51ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a
encrypted: [4, 37, 3e, ed, d7, 8a, ad, 9b, 95, d5, 66, 4a, 1d, 4b, 75, f, 0, ed, b1, 3b, 72, 2a, d6, af, a3, c9, 5c, 1e, 35, 2a, 10, b5, ae, d3, 91, aa, f7, c9, 40, 8a, 95, 40, f1, 21, cb, e7, 2e, 6c, 1d, 13, 46, 32, 5f, 1d, b9, 33, 2f, 8e, f2, 39, 8b, 9c, da, 29, 43, 50, e1, d4, 4e, 3c, 6e, 87, b7, 1b, 5c, 83, 90, 12, e9, 86, 51, 8a, 90, 73, 2c, bf, 3a, fe, 32, f3, 4, 31, 6c, 9b, 59, 4f, 4, d7, c5, 64, 64, 73, dd, 79, c1, 45, ac]

encrypted: 04373eedd78aad9b95d5664a1d4b750f00edb13b722ad6afa3c95c1e352a10b5aed391aaf7c9408a9540f121cbe72e6c1d1346325f1db9332f8ef2398b9cda294350e1d44e3c6e87b71b5c839012e986518a90732cbf3afe32f304316c9b594f04d7c5646473dd79c145ac
decrypted: helloworld
```

## Public Key

Get Public Key for a Private Key: 
```
% cargo run pubkey   

Public Key in Hex: 04c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee51ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a8767a516f3c807a38fc94fad34043c569fcc97ae2944b8c06592a5f84b1b39f611b430140a03af3ca94ab0d738cfc0af6b0bcb44e5ba
```

## Encrypt Message  
Supply the ```Public Key``` as the first argument to the ```encrypt``` subcommand with the message to encrypt as the last argument.  
```
% cargo run encrypt 04c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee51ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a helloworld

Message encrypted: 04aa5f8d18e275f63e9e1c48b138b913e22ad9cf74468d1e49f7a35cd36449c554e5943ecfe7bdc5e420407eeb121e2518d156eb2d787723d3395951f41c4c566cc318561a4324a20e8213cf571ae8642dae095a81221c4b910197a59fbc6ab36f2b0bb6619a78f10acb94
```  

## Decrypt Message 
To decrypt the encrypted data provide it as an argument to ```--decrypt```
```
% cargo run decrypt 04aa5f8d18e275f63e9e1c48b138b913e22ad9cf74468d1e49f7a35cd36449c554e5943ecfe7bdc5e420407eeb121e2518d156eb2d787723d3395951f41c4c566cc318561a4324a20e8213cf571ae8642dae095a81221c4b910197a59fbc6ab36f2b0bb6619a78f10acb94

Decrypted message: helloworld
```  
## Encrypt File 
Similarly for files we need to pass the ```Public Key``` as an argument to the ```encrypt-file``` subcommand with a file path for the file to encrypt as well as the output file path.  
```
% cargo run encrypt-file 04c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee51ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a test123 test123.enc

File encrypted successfully and saved to test123.enc
```

## Decrypt File
```
% cargo run decrypt-file test123.enc test123.out
Decryption complete. Decrypted data written to: test123.out
% cat test123.out 
test
```