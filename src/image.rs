use std::fmt;
use std::fmt::{Display, Formatter};
use std::process::{Output, Command};
use crate::buildah_error::BuildahError;

/// ...
#[derive(Debug, Clone)]
pub struct Image(String);

/// ...
///
/// Ideally `pub const SCRATCH: Image = Image::new(None, "SCRATCH", None);`.
/// However, "function pointer casts are not allowed in constant functions".
pub const SCRATCH: &str = "scratch";

// TODO: codegen a bunch of constants: fetch built-in images and all their tags
//  buildah::image::ubuntu::16_04, buildah::image::alpine::LATEST
//  run daily, only re-publish if anything changed

impl Image {
    /// Create an image from its components.
    pub fn new(hostname: Option<&str>, name: &str, tag: Option<&str>) -> Self {
        Image(match (hostname, tag) {
            (Some(registry), Some(tag)) => format!("{}/{}:{}", registry, name, tag),
            (Some(registry), None) => format!("{}/{}", registry, name),
            (None, Some(tag)) => format!("{}:{}", name, tag),
            (None, None) => format!("{}", name),
        })
    }

    /// Push an image, manifest list or image index from local storage to elsewhere.
    /// Currently equivalent to docker push command. Missing lots of options.
    pub fn push(&self) -> Result<Output, BuildahError> {
        Command::new("buildah")
            .arg("push")
            .arg(self.to_string())
            .output()
            .map_err(BuildahError::from)
    }

    // TODO: pull command

    // TODO: with_hostname(Option<&str>), with_name(&str), with_tag(Option<&str>)
    //  implement same try_from pattern as container
    //  store internally as registry, name, tag; see SCRATCH 7 how to do it with nom!
}

impl<T: Into<String>> From<T> for Image {
    fn from(string: T) -> Self {
        Image(string.into())
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
