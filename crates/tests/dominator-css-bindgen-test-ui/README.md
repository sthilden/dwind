# DWIND example UI

## Running the example UI

You need a recent (>= 1.79.0) version of rust, please refer to https://rustup.rs/ for how to install this.

After installing those, add the `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```

You now have two ways of running the application

### rollup/npm

You also need npm and node installed, see https://github.com/nvm-sh/nvm for installation instructions

```shell
npm install
npm start
```

### Trunk

First install the trunk utility: https://trunkrs.dev/guide/getting-started/installation.html

then do

```shell
trunk serve --open
```