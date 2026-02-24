use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const ICONS_REL_PATH: &str = "extensions/iconpacks/jonathanharty.gruvbox-material-icon-theme-1.1.5/icons";
const ICONS_JSON_REL_PATH: &str = "extensions/iconpacks/jonathanharty.gruvbox-material-icon-theme-1.1.5/dist/material-icons.json";

fn icons_base() -> PathBuf {
    crate::resources::resource_dir().join(ICONS_REL_PATH)
}

#[derive(Deserialize)]
struct IconTheme {
    #[serde(rename = "fileExtensions")]
    file_extensions: HashMap<String, String>,
    #[serde(rename = "fileNames")]
    file_names: HashMap<String, String>,
    #[serde(rename = "folderNames")]
    folder_names: HashMap<String, String>,
    #[serde(rename = "folderNamesExpanded")]
    folder_names_expanded: HashMap<String, String>,
    file: String,
    folder: String,
    #[serde(rename = "folderExpanded")]
    folder_expanded: String,
}

static ICON_THEME: Lazy<Option<IconTheme>> = Lazy::new(|| {
    let json_path = crate::resources::resource_dir().join(ICONS_JSON_REL_PATH);
    let json_content = fs::read_to_string(&json_path).ok()?;
    serde_json::from_str(&json_content).ok()
});

fn icon_path(name: &str) -> String {
    icons_base().join(format!("{}.svg", name)).to_string_lossy().into_owned()
}

pub fn get_file_icon(filename: &str) -> String {
    let filename_lower = filename.to_lowercase();

    if let Some(theme) = ICON_THEME.as_ref() {
        if let Some(icon_name) = theme.file_names.get(&filename_lower) {
            return icon_path(icon_name);
        }

        let parts: Vec<&str> = filename_lower.split('.').collect();
        for i in 1..parts.len() {
            let compound_ext = parts[i..].join(".");
            if let Some(icon_name) = theme.file_extensions.get(&compound_ext) {
                return icon_path(icon_name);
            }
        }

        if let Some(ext) = Path::new(filename).extension().and_then(|e| e.to_str()) {
            if let Some(icon_name) = theme.file_extensions.get(&ext.to_lowercase()) {
                return icon_path(icon_name);
            }
        }

        return icon_path(&theme.file);
    }

    icon_path("file")
}

pub fn get_folder_icon(folder_name: &str, is_open: bool) -> String {
    let folder_lower = folder_name.to_lowercase();

    if let Some(theme) = ICON_THEME.as_ref() {
        if is_open {
            if let Some(icon_name) = theme.folder_names_expanded.get(&folder_lower) {
                return icon_path(icon_name);
            }
            return icon_path(&theme.folder_expanded);
        } else {
            if let Some(icon_name) = theme.folder_names.get(&folder_lower) {
                return icon_path(icon_name);
            }
            return icon_path(&theme.folder);
        }
    }
    let suffix = if is_open { "-open" } else { "" };
    icon_path(&format!("folder{}", suffix))
}
