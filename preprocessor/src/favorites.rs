use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;

lazy_static! {
    // Match :favorites [...] in EDN
    static ref FAVORITES_RE: Regex = Regex::new(r":favorites\s+\[([\s\S]*?)\]").unwrap();

    // Match individual items in favorites list
    static ref FAV_ITEM_RE: Regex = Regex::new(r#""([^"]+)""#).unwrap();

    // Match :default-home {:page "..."} in EDN
    static ref DEFAULT_HOME_RE: Regex = Regex::new(r#":default-home\s+\{[^}]*:page\s+"([^"]+)""#).unwrap();

    // Match :meta/title "..." in EDN (optional site title)
    static ref SITE_TITLE_RE: Regex = Regex::new(r#":meta/title\s+"([^"]+)""#).unwrap();
}

/// Process favorites from logseq/config.edn
pub fn process_favorites(
    config_path: &Path,
    favorites_output: &Path,
    pages_output: &Path,
) -> Result<usize> {
    let content = fs::read_to_string(config_path)?;

    // Extract favorites list
    let favorites = extract_favorites(&content);
    if favorites.is_empty() {
        return Ok(0);
    }

    // Create favorites index
    let mut index_content = String::from("---\ntitle: \"⭐ Favorites\"\n---\n\n");

    let mut count = 0;
    for fav in &favorites {
        // Page files use lowercase names with spaces preserved
        let page_filename = fav.to_lowercase();
        let slug = slugify(fav);

        // Check if page exists (with original spacing)
        let page_path = pages_output.join(format!("{}.md", page_filename));
        if !page_path.exists() {
            continue;
        }

        // Get icon from page if exists
        let icon = get_page_icon(&page_path).unwrap_or_default();

        // Create favorite embed file (use slugified name for URL-safe paths)
        let fav_path = favorites_output.join(format!("{}.md", slug));
        let fav_content = format!(
            "---\ntitle: \"{}{}\"\n---\n\n![[{}]]\n",
            if icon.is_empty() { String::new() } else { format!("{} ", icon) },
            fav,
            page_filename
        );

        fs::write(&fav_path, fav_content)?;
        count += 1;

        // Add to index
        index_content.push_str(&format!(
            "- [[favorites/{}|{}{}]]\n",
            slug,
            if icon.is_empty() { String::new() } else { format!("{} ", icon) },
            fav
        ));
    }

    // Write index
    fs::write(favorites_output.join("index.md"), index_content)?;

    Ok(count)
}

/// Extract favorites from config.edn content
fn extract_favorites(content: &str) -> Vec<String> {
    let mut favorites = Vec::new();

    if let Some(caps) = FAVORITES_RE.captures(content) {
        let list = caps.get(1).unwrap().as_str();

        for item in FAV_ITEM_RE.captures_iter(list) {
            let fav = item.get(1).unwrap().as_str().to_string();
            favorites.push(fav);
        }
    }

    favorites
}

/// Convert page name to slug (matching page filename format)
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('/', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect()
}

/// Get icon from page frontmatter or properties
fn get_page_icon(page_path: &Path) -> Option<String> {
    let content = fs::read_to_string(page_path).ok()?;

    // Check for icon:: property
    for line in content.lines().take(20) {
        let line = line.trim().trim_start_matches('-').trim();
        if let Some(rest) = line.strip_prefix("icon::") {
            return Some(rest.trim().to_string());
        }
        if let Some(rest) = line.strip_prefix("icon:") {
            return Some(rest.trim().trim_matches('"').to_string());
        }
    }

    None
}

/// Extract default home page from config.edn
/// Returns the page name from :default-home {:page "..."}
pub fn get_default_home(config_path: &Path) -> Option<String> {
    let content = fs::read_to_string(config_path).ok()?;

    // Process line by line to skip comments
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip commented lines
        if trimmed.starts_with(";;") || trimmed.starts_with(";") {
            continue;
        }
        // Check for :default-home on this line
        if let Some(caps) = DEFAULT_HOME_RE.captures(line) {
            return Some(caps.get(1)?.as_str().to_string());
        }
    }

    None
}

/// Extract site title from config.edn
/// Tries :meta/title first, then falls back to :default-home page name
pub fn get_site_title(config_path: &Path) -> Option<String> {
    let content = fs::read_to_string(config_path).ok()?;

    // Process line by line to skip comments
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip commented lines
        if trimmed.starts_with(";;") || trimmed.starts_with(";") {
            continue;
        }
        // Check for :meta/title first
        if let Some(caps) = SITE_TITLE_RE.captures(line) {
            return Some(caps.get(1)?.as_str().to_string());
        }
    }

    // Fall back to default-home page name
    get_default_home(config_path)
}

/// Site configuration extracted from Logseq config
#[derive(serde::Serialize)]
pub struct SiteConfig {
    pub page_title: String,
    pub home_page: String,
}

/// Write site configuration to JSON file for Quartz config generation
pub fn write_site_config(config_path: &Path, output_dir: &Path) -> Option<SiteConfig> {
    let home_page = get_default_home(config_path).unwrap_or_else(|| "index".to_string());
    let page_title = get_site_title(config_path).unwrap_or_else(|| home_page.clone());

    let site_config = SiteConfig {
        page_title: capitalize_first(&page_title),
        home_page: home_page.clone(),
    };

    // Write to JSON file
    let config_json = serde_json::to_string_pretty(&site_config).ok()?;
    fs::write(output_dir.join("_site_config.json"), config_json).ok()?;

    Some(site_config)
}

/// Capitalize first letter of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
