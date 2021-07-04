use std::fmt::Debug;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use tracing::{error, info};

use crate::buildah_error::BuildahError;
use crate::image::Image;
use crate::mount::Mount;
use crate::parsing::parse_output_line;

/// Instance of a Buildah container.
/// Removed on drop.
#[derive(Debug)]
pub struct Container {
    name: String,
}

impl Container {
    /// Creates a new working container, either from scratch or using a specified image as a starting point.
    ///
    /// Ideally this would be `impl<T: Into<Image>> TryFrom<T> for Container {...}`.
    /// However, core Rust includes the blanket implementation `impl<T, U> TryFrom<U> for T where U: Into<T>`.
    /// This causes a name conflict, and hence this workaround is necessary.
    ///
    /// Into<Image> generics must be Debug for instrumentation.
    #[tracing::instrument]
    pub fn try_from<T: Into<Image> + Debug>(image: T) -> Result<Self, BuildahError> {
        let image = image.into();

        let output = Command::new("buildah")
            .arg("from")
            .arg(image.to_string())
            .output()?;

        let name = parse_output!(output);
        info!("Created container {}", name);
        Ok(Container { name: String::from(name) })
    }

    /// Typed implementation of [`config`] method...
    pub fn config_author(&mut self, name: &str) -> Result<(), BuildahError> {
        self.config("author", name)
    }

    /// Typed implementation of [`config`] method...
    pub fn config_workingdir<P: AsRef<Path>>(&mut self, directory: P) -> Result<(), BuildahError> {
        self.config("workingdir", directory.as_ref().to_str().unwrap())
    }

    /// Typed implementation of [`config`] method...
    pub fn config_cmd(&mut self, command: &str) -> Result<(), BuildahError> {
        self.config("cmd", command)
    }

    /// Typed implementation of [`config`] method...
    pub fn config_port(&mut self, port: u16) -> Result<(), BuildahError> {
        self.config("cmd", &port.to_string())
    }

    /// Update image configuration settings.
    /// TODO: support more variations.
    #[tracing::instrument]
    fn config(&mut self, key: &str, value: &str) -> Result<(), BuildahError> {
        let output = Command::new("buildah")
            .arg("config")
            .arg(format!("--{}={}", key, value))
            .arg(&self.name)
            .output()?;

        let _ = parse_output!(output);
        Ok(())
    }

    /// Copies the contents of a file, URL, or directory into container's working directory.
    /// TODO: support more variations. TODO: try_copy
    ///
    /// AsRef<Path> generics must be Debug for instrumentation.
    #[tracing::instrument]
    pub fn copy<P1: AsRef<Path> + Debug, P2: AsRef<Path> + Debug>(&mut self, src: P1, dest: P2) -> Result<(), BuildahError> {
        let output = Command::new("buildah")
            .arg("copy")
            .arg(&self.name)
            .arg(src.as_ref())
            .arg(dest.as_ref())
            .output()?;

        let _id = parse_output!(output);
        info!("Copied");
        Ok(())
    }

    /// Run a command inside of the container.
    #[tracing::instrument]
    pub fn run(&mut self, args: &[&str]) -> Result<Output, BuildahError> {
        let output = Command::new("buildah")
            .arg("run")
            .arg(&self.name)
            .arg("--")
            .args(args)
            .output()?;

        info!("Ran");
        Ok(output)
    }

    /// Mount working container's root filesystem.
    /// Unprivileged users must run with `buildah unshare`.
    pub fn mount(&mut self) -> Mount {
        Mount::new(&self.name).unwrap() // TODO: add try_mount? Buildah("cannot mount using driver overlay in rootless mode. You need to run it in a `buildah unshare` session")',
    }

    /// Create an image from a working container.
    /// TODO: support more variations.
    ///
    /// Into<Image> generics must be Debug for instrumentation.
    #[tracing::instrument]
    pub fn commit<T: Into<Image> + Debug>(&mut self, image: T) -> Result<(), BuildahError> {
        let image = image.into();

        let output = Command::new("buildah")
            .arg("commit")
            .arg(&self.name)
            .arg(image.to_string())
            .output()?;

        let _id = parse_output!(output);
        info!("Committed as image {}", image.to_string());
        Ok(())
    }
}

impl<T: Into<Image> + Debug> From<T> for Container {
    fn from(image: T) -> Self {
        Container::try_from(image).unwrap()
    }
}

/// Removes working container.
impl Drop for Container {
    #[tracing::instrument]
    fn drop(&mut self) {
        let output = Command::new("buildah")
            .arg("rm")
            .arg(&self.name)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    // Ignore error
                    let error = BuildahError::Buildah(parse_output_line(output.stderr));
                    error!("Couldn't remove container {}: {:?}", self.name, error);
                    return;
                }

                let _id = parse_output_line(output.stdout);
                info!("Removed container {}", self.name);
            }
            Err(error) => {
                // Ignore error
                error!("Couldn't remove container {}: {:?}", self.name, error);
            }
        }
    }
}
