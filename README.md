```
cd dart
dart run ffigen --config ffigen-wasmtime.yaml
```

# TODO

- scope wasmtime to functions that we're using
- load wasmtime.so
    - when using the llvm-path it currently errors telling me no file is found
- implement the host for plugin
- dynamically load a plugin