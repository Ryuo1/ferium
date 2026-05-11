use ferinth::structures::version::{Version, VersionFile};

pub trait VersionExt {
    /// Gets the primary (or first) version file of a version
    fn get_version_file(&self) -> &VersionFile;

    /// Consumes and returns the primary (or first) version file of a version
    fn into_version_file(self) -> VersionFile;
}

impl VersionExt for Version {
    fn get_version_file(&self) -> &VersionFile {
        if self.files.is_empty() {
            println!("================ EMPTY FILES DETECTED ================");
            println!("{:#?}", self);
            panic!("Version has no files");
        }

        self.files
            .iter()
            .find(|f| f.primary)
            .unwrap_or_else(|| &self.files[0])
    }

    fn into_version_file(mut self) -> VersionFile {
        if self.files.is_empty() {
            println!("================ EMPTY FILES DETECTED ================");
            println!("{:#?}", self);
            panic!("Version has no files");
        }

        let fallback = self.files.swap_remove(0);

        self.files
            .into_iter()
            .find(|f| f.primary)
            .unwrap_or(fallback)
    }
}
