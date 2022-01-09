# OSM Password Hash

A practice work of programming in Rust. This program takes the first argument as a password, and generates a salt and hash that can be used as an OpenStreetMap user credential stored in the database.

Basically, OSM generates user credentials with PBKDF2 with HMAC-SHA512 algorithm to derive the hash string. This program simply does what OSM does.

As to why I wrote this, the Ruby based OSM source code doesn't provide an API for user registration which makes it necessary for such a program when I need to manage some private OSM instances, it'll be easier to manipulate user credentials in bulk.

> Disclaimer: this project is not affiliated with OpenStreetMap.

# Usage

```bash
cargo run your-password-here
```

# License

GPLv3 © [mogita](https://github.com/mogita)
