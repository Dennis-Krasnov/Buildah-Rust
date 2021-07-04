# Buildah

Wrapper around the [Buildah](https://github.com/containers/buildah) CLI tool. For people who prefer Rust over Bash.

For more advanced use cases, use the buildah Go library itself.

## Example
```shell
cd examples
 
# Buildah Rust
cargo run --example nginx
podman run --rm -it -p 8080:80 nginx_rust

# Buildah dockerfile
buildah bud -f nginx_dockerfile -t nginx_bud .
podman run --rm -it -p 8080:80 nginx_bud

# Buildah Bash
chmod u+x nginx.sh
./nginx.sh
podman run --rm -it -p 8080:80 nginx_bash

# Check that it worked:
buildah images
```

## TODO
[] Validate image names

[] Better error handling, better logging, panic if buildah isn't installed

[] Prelude: `use buildah::prelude::*;`

[] Publish on crates.io, ask `buildah` owner if I can have it, otherwise `buildah-rs`

[] Lots of commands and options are missing!

[] The trait `std::error::Error` is not implemented for `BuildahError`

[] Example with chrome tracing

[] https://rust-lang.github.io/api-guidelines/checklist.html

[] https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate

[] write docs on how to run this on a macos/windows machine
//  1) locally, `cargo build --release --package build_oci_images`
//  2) docker run --rm -v "$PWD":/app -w /app -e IMAGE_TAG=latest buildah/buildah buildah unshare target/release/build_oci_images
//  https://insujang.github.io/2020-11-09/building-container-image-inside-container-using-buildah/
//  https://developers.redhat.com/blog/2019/08/14/best-practices-for-running-buildah-in-a-container#running_buildah_inside_a_container

## License

Licensed under Apache License, Version 2.0.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
