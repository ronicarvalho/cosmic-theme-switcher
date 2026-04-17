use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn opposite(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Light => b"false",
            Self::Dark => b"true",
        }
    }
}

const REL_PATH: &str = "cosmic/com.system76.CosmicTheme.Mode/v1/is_dark";

pub fn is_dark_path() -> PathBuf {
    let base = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"));
    base.join(REL_PATH)
}

pub fn read() -> ThemeMode {
    read_from(&is_dark_path())
}

pub fn write(mode: ThemeMode) -> io::Result<()> {
    write_to(&is_dark_path(), mode)
}

pub fn read_from(path: &Path) -> ThemeMode {
    match fs::read_to_string(path) {
        Ok(contents) => match contents.trim() {
            "true" => ThemeMode::Dark,
            "false" => ThemeMode::Light,
            other => {
                tracing::warn!(value = %other, path = %path.display(), "unrecognized is_dark value, defaulting to Light");
                ThemeMode::Light
            }
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => ThemeMode::Light,
        Err(err) => {
            tracing::warn!(?err, path = %path.display(), "failed to read is_dark, defaulting to Light");
            ThemeMode::Light
        }
    }
}

pub fn write_to(path: &Path, mode: ThemeMode) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, mode.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn tmp_file() -> (TempDir, PathBuf) {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("nested/dir/is_dark");
        (dir, path)
    }

    #[test]
    fn read_missing_file_defaults_to_light() {
        let (_dir, path) = tmp_file();
        assert_eq!(read_from(&path), ThemeMode::Light);
    }

    #[test]
    fn read_true_is_dark() {
        let (_dir, path) = tmp_file();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "true").unwrap();
        assert_eq!(read_from(&path), ThemeMode::Dark);
    }

    #[test]
    fn read_false_is_light() {
        let (_dir, path) = tmp_file();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "false").unwrap();
        assert_eq!(read_from(&path), ThemeMode::Light);
    }

    #[test]
    fn read_trims_surrounding_whitespace() {
        let (_dir, path) = tmp_file();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "  true\n").unwrap();
        assert_eq!(read_from(&path), ThemeMode::Dark);
    }

    #[test]
    fn read_garbage_defaults_to_light() {
        let (_dir, path) = tmp_file();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "banana").unwrap();
        assert_eq!(read_from(&path), ThemeMode::Light);
    }

    #[test]
    fn write_creates_parent_dirs() {
        let (_dir, path) = tmp_file();
        write_to(&path, ThemeMode::Dark).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn write_dark_persists_exactly_true() {
        let (_dir, path) = tmp_file();
        write_to(&path, ThemeMode::Dark).unwrap();
        let bytes = fs::read(&path).unwrap();
        assert_eq!(bytes, b"true");
    }

    #[test]
    fn write_light_persists_exactly_false() {
        let (_dir, path) = tmp_file();
        write_to(&path, ThemeMode::Light).unwrap();
        let bytes = fs::read(&path).unwrap();
        assert_eq!(bytes, b"false");
    }

    #[test]
    fn toggle_light_to_dark_via_write() {
        let (_dir, path) = tmp_file();
        write_to(&path, ThemeMode::Light).unwrap();
        assert_eq!(read_from(&path), ThemeMode::Light);
        write_to(&path, ThemeMode::Light.opposite()).unwrap();
        assert_eq!(read_from(&path), ThemeMode::Dark);
    }

    #[test]
    fn toggle_dark_to_light_via_write() {
        let (_dir, path) = tmp_file();
        write_to(&path, ThemeMode::Dark).unwrap();
        assert_eq!(read_from(&path), ThemeMode::Dark);
        write_to(&path, ThemeMode::Dark.opposite()).unwrap();
        assert_eq!(read_from(&path), ThemeMode::Light);
    }

    #[test]
    fn opposite_is_involutive() {
        assert_eq!(ThemeMode::Light.opposite(), ThemeMode::Dark);
        assert_eq!(ThemeMode::Dark.opposite(), ThemeMode::Light);
        assert_eq!(ThemeMode::Light.opposite().opposite(), ThemeMode::Light);
    }
}
