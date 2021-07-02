use std::path::{Path, PathBuf};
use std::process::Command;

use tracing::{error, info};

use crate::buildah_error::BuildahError;
use crate::parsing::parse_output_line;

/// Instance of a Buildah mount.
/// Unmounted on drop.
#[derive(Debug)]
pub struct Mount {
    container_name: String,
    host_path: PathBuf,
}

impl Mount {
    /// Mount a working container's root filesystem.
    #[tracing::instrument]
    pub fn new(container_name: &str) -> Result<Self, BuildahError> {
        let output = Command::new("buildah")
            .arg("mount")
            .arg(container_name)
            .output()?;

        let path = parse_output!(output);
        info!("Mounted container {}", container_name);
        Ok(Mount { container_name: container_name.to_string(), host_path: PathBuf::from(path) })
    }

    /// ...
    pub fn host_path(&self) -> &Path {
        self.host_path.as_path()
    }
}

/// Unmount the root file system on the working container.
impl Drop for Mount {
    #[tracing::instrument]
    fn drop(&mut self) {
        let output = Command::new("buildah")
            .arg("umount")
            .arg(&self.container_name)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    // Ignore error
                    let error = BuildahError::Buildah(parse_output_line(output.stderr));
                    error!("Couldn't umount container {}: {:?}", self.container_name, error);
                    return;
                }

                let _id = parse_output_line(output.stdout);
                info!("Umounted container {}", self.container_name);
            }
            Err(error) => {
                // Ignore error
                error!("Couldn't umount container {}: {:?}", self.container_name, error);
            }
        }
    }
}
