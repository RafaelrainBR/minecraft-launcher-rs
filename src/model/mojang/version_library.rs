use crate::launcher_paths::join_paths;

#[derive(Debug, Clone)]
pub struct VersionLibrary {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    pub is_native: bool,
}

impl VersionLibrary {
    pub fn new(sha1: String, size: u64, url: String, is_native: bool) -> Self {
        Self {
            sha1,
            size,
            url,
            is_native,
        }
    }

    pub fn is_native(&self) -> bool {
        self.is_native
    }

    pub fn get_path(&self) -> String {
        Self::extract_path_from_url(&self.url)
    }

    fn extract_path_from_url(url: &str) -> String {
        let mut path_parts: Vec<&str> = url.split('/').skip(3).collect();

        join_paths(path_parts.remove(0).to_owned(), path_parts).to_string()
    }
}
