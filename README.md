# dgraph-js-native

dGraph JS Native Client

The motivation behind this client is to eliminate performance bottleneck between NodeJS gRPC related libraries and dGraph by wrapping a native gRPC client ([dgraph-tonic](https://github.com/selmeci/dgraph-tonic)) and using in NodeJS. Thanks to powerful tools like [neon](https://github.com/neon-bindings/neon) it is relatively painless.

# Installation

```
npm install dgraph-js-native --save
# or
yarn add dgraph-js-native
```

# Usage

I am trying to keep the API contract as close to `dgraph-js` as possible, currently there is a notable difference in the client initialization code, where I abstracted out the client stub initialization.

TODO: Example

# Benchmark

[Benchmark Test Repo](https://github.com/xanthous-tech/dgraph-js-native-benchmarks)

# Sponsor

[Treelab](https://treelab.com.cn)

# License

[MIT](./LICENSE)
