# This is the rust service that acts as a gateway

## run
```bash
./build.sh

cargo run --bin server

or 

proxychains4 cargo run --bin server #For Chinese
```

## test
```bash
cargo test -- --color always --nocapture
```

## Build
```bash
docker build --tag weloveparty_gateway .

docker run --rm --name weloveparty_gateway \
weloveparty_gateway
```