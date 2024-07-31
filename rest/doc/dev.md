# Prepare development environment

```bash
sudo apt install libmysqlclient-dev
cargo install diesel_cli --no-default-features --features mysql

cd doker 
docker compose up -d
cd ..
diesel migration run
```

# Run

```bash
export RUST_LOG="debug"
cargo watch -x 'run --bin rest'
```


