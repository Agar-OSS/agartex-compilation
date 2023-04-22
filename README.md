# agartex-service

Service with backend functionalities for AgarTeX

## Runbook

To run locally from repository root use

```
cargo run
```

To run tests use
```
cargo test
```

To run linting use
```
cargo clippy --all-targets --all-features --fix -- -D warnings
```

## Docker

### Build
```
docker build -t agaross.azurecr.io/agar-oss/latex-base latex
docker build -t agaross.azurecr.io/agar-oss/agartex-compilation .
```
