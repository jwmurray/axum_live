
## Watching the src/ tree:
```
cargo watch -q -c -w src/ -x run
```

## Running test and example code.  Example code is cleaner, but the test code is more naturally included in the CI/CD integration acceptance criteria

## watching the examples/ tree:  (quick_dev.rs in examples folder with a tokio::main and main() function)

```
cargo watch -q -c -w examples/ -w src/ -x  'run --example quick_dev'
```

## watching the tests/ and src/ folders:  (quick_dev.rs in tests folder with a #[tokion::test] and function named quick_dev() )
```
cargo watch -q -c -w tests/ -w src/ -x  "test -q quick_dev -- --nocapture"
```