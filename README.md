# Buildah

Wrapper around the [Buildah](https://github.com/containers/buildah) CLI tool. For people who prefer Rust over Bash.

For more advanced use cases, use the buildah Go library itself.

## Example
```shell
cd examples
buildah images

# nginx_dockerfile
buildah bud -f nginx_dockerfile -t nginx_bud .
podman run --rm -it -p 8080:80 nginx_bud
 
# Buildah rust
cargo run --example nginx
podman run --rm -it -p 8080:80 nginx_rust

# Buildah Rust
chmod u+x nginx.sh
./nginx.sh
podman run --rm -it -p 8080:80 nginx_bash
```

## TODO
[] Validate image names

[] Better error handling, better logging, panic if buildah isn't installed

[] Prelude: `use buildah::prelude::*;`

[] Publish on crates.io, ask `buildah` owner if I can have it, otherwise `buildah-rs`

[] Lots of commands and options are missing!

[] The trait `std::error::Error` is not implemented for `BuildahError`

[] Example with chrome tracing
