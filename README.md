## Installation

### Check the prerequisites
**Please ensure**…
- … that [`Node.js`](https://nodejs.org/en/download) is installed (14.21.3, or more recent);
- … that [`pnpm`](https://pnpm.io/installation) is installed;
- … and that [`Rust`](https://www.rust-lang.org/tools/install) is installed. This installation will include `cargo`.

**Using `cargo`, install the prerequisite crates.**
This software needs [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer) and [`rsw`](https://github.com/rwasm/rsw-rs).
```
cargo install wasm-pack rsw
```

### Install the `Node.js` dependencies
```
pnpm i
```

---

## Usage
To run the development server:
```
pnpm dev:rsw
```
To build the site for deployment:
```
pnpm build:rsw
```

---

## Acknowledgement

Thanks to Julian Cataldo for making it possible that `nannou` and `astro` work together in this project; it was built on the boilerplate code from this [repository](https://github.com/JulianCataldo/astro-nannou-starter/commit/6194e8e52afe12b2036490d1e14d244457129269). Visit [JulianCataldo.com](https://www.juliancataldo.com).
