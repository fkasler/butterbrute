# ButterBrute

ButterBrute is a Buttercup password vault brute forcing utility. This tool implements a dictionary attack to crack the master password on buttercup (.bcup) files.

## Build/Installation

The project includes code for a Rust binary to perform the dictionary attack, and a Node.js script to read out the vault contents once you crack the password. Needless to say, will need Rust and Node installed to use this repository.

Build the brute forcer:
```
cargo build --release
```

If you want to dump the vault contents once you crack the password, you can use dump_vault.mjs, but you will need to install dependencies with:
```
npm install
```

## Attack!!!

Run the dictionary attack:
```
./target/release/butterbrute testvault.bcup testwordlist.txt
```

Once you crack the password, you can dump the vault contents with:
```
node dump_vault.mjs testvault.bcup password1
```

# Why?

It didn't seem like anyone else had built one yet. While working on a CTF, I came across a vault.bcup file and just needed to know what was inside. There was no hashcat mode for cracking buttercup, so I solved the CTF with a PoC in Node.js using the buttercup-core library. It got the job done, but it was extremely inefficient, and I would not use it on a real pen test. What's worth doing is worth overdoing, so I dug into the crypto and built an efficient brute forcer in Rust. It was a fun learning experience, and I found out that Buttercup and all it's crypto is a basically an elaborate Princess Bride reference. Anyway...

![A gif of a movie reference](./sendoff.gif)
