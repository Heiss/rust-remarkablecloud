# Oxidized reMarkable cloud

Get your reMarkable connect data back in your hand.

## Status

This is not ready to use.

## Inspiration

This implementation is heavily inspired by the excellent work of [ddvk](https://ddvk.github.io/rmfakecloud/).

## Development

It is very easy to get a dev environment. You need `git`, `npm`, `node`, `rustup`, `cargo`, `rust`. After this you can execute the following commands to get everything up and running.

```bash
git clone https://github.com/Heiss/rust-remarkablecloud.git
cd rust-remarkablecloud

cargo install cargo-make cargo-watch
cargo make install
cargo make dev
```

After this, you will have both running, the npm dev frontend server and the rust api backend server. Both will restart recognize the corresponding folders and files to check.
