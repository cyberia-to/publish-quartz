use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::page::PageIndex;
use crate::query;

lazy_static! {
    // Logseq system properties to remove completely (not user data)
    static ref SYSTEM_PROPS_RE: Regex = Regex::new(
        r"(?m)^(\s*)(?:-\s*)?(collapsed|logseq\.order-list-type|id|query-table|query-sort-by|query-sort-desc|query-properties):: .+$"
    ).unwrap();

    // User inline properties to convert to readable format (key:: value → **Key:** value)
    static ref USER_PROPS_RE: Regex = Regex::new(
        r"(?m)^(\s*)(?:-\s*)?([\w-]+):: (.+)$"
    ).unwrap();

    // Logseq image size attributes {:height N, :width N}
    static ref IMAGE_SIZE_RE: Regex = Regex::new(r"\{:height\s+\d+,?\s*:width\s+\d+\}").unwrap();

    // Empty bullet lines (just "- " or "-" with optional whitespace)
    static ref EMPTY_BULLET_RE: Regex = Regex::new(r"(?m)^(\s*)-\s*$").unwrap();

    // Wikilinks with $ signs
    static ref WIKILINK_DOLLAR_RE: Regex = Regex::new(r"\[\[([^\]]*\$[^\]]*)\]\]").unwrap();

    // Standalone $ tokens (matches $TOKEN patterns)
    static ref DOLLAR_TOKEN_RE: Regex = Regex::new(r"(^|[^\\])\$([A-Z][A-Z0-9]*)").unwrap();

    // Embed syntax
    static ref EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\[\[([^\]]+)\]\]\s*\}\}").unwrap();

    // Block embed
    static ref BLOCK_EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\(\(([^)]+)\)\)\s*\}\}").unwrap();

    // Block reference
    static ref BLOCK_REF_RE: Regex = Regex::new(r"\(\(([a-f0-9-]{36})\)\)").unwrap();

    // Query syntax
    static ref QUERY_RE: Regex = Regex::new(r"(?ms)^(\s*-\s*)?\{\{query[\s\S]*?\}\}").unwrap();

    // YouTube/video/pdf embeds
    static ref YOUTUBE_RE: Regex = Regex::new(r"\{\{youtube\s+([^\}]+)\}\}").unwrap();
    static ref VIDEO_RE: Regex = Regex::new(r"\{\{video\s+([^\}]+)\}\}").unwrap();
    static ref PDF_RE: Regex = Regex::new(r"\{\{pdf\s+([^\}]+)\}\}").unwrap();

    // Renderer
    static ref RENDERER_RE: Regex = Regex::new(r"\{\{renderer\s+[^\}]+\}\}").unwrap();

    // Cloze
    static ref CLOZE_RE: Regex = Regex::new(r"\{\{cloze\s+([^\}]+)\}\}").unwrap();

    // Hiccup/EDN syntax (Clojure-style [:tag ...] blocks) - matches balanced brackets
    static ref HICCUP_LINE_RE: Regex = Regex::new(r"(?m)^(\s*-\s*)?\[:\w").unwrap();

    // Extract text content from hiccup
    static ref HICCUP_TEXT_RE: Regex = Regex::new(r#""([^"]+)""#).unwrap();
    static ref HICCUP_H2_RE: Regex = Regex::new(r#"\[:h2\s+"([^"]+)"\]"#).unwrap();
    static ref HICCUP_H3_RE: Regex = Regex::new(r#"\[:h3\s+"([^"]+)"\]"#).unwrap();
    static ref HICCUP_LI_RE: Regex = Regex::new(r#"\[:li\s+"([^"]+)"\]"#).unwrap();

    // Task markers
    static ref DONE_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+DONE\s+").unwrap();
    static ref TODO_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+TODO\s+").unwrap();
    static ref NOW_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+NOW\s+").unwrap();
    static ref DOING_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+DOING\s+").unwrap();
    static ref LATER_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+LATER\s+").unwrap();
    static ref WAITING_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+WAITING\s+").unwrap();
    static ref CANCELLED_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+CANCELLED\s+").unwrap();

    // Priority markers
    static ref PRIORITY_A_RE: Regex = Regex::new(r"\[#A\]").unwrap();
    static ref PRIORITY_B_RE: Regex = Regex::new(r"\[#B\]").unwrap();
    static ref PRIORITY_C_RE: Regex = Regex::new(r"\[#C\]").unwrap();

    // Schedule/deadline
    static ref SCHEDULED_RE: Regex = Regex::new(r"SCHEDULED:\s*<([^>]+)>").unwrap();
    static ref DEADLINE_RE: Regex = Regex::new(r"DEADLINE:\s*<([^>]+)>").unwrap();

    // Wikilinks (for adding pages/ prefix)
    static ref WIKILINK_RE: Regex = Regex::new(r"(!\s*)?\[\[([^\]|]+)(\|[^\]]*)?\]\]").unwrap();

    // Tables - fix pipe in first cell
    static ref TABLE_FIRST_CELL_RE: Regex = Regex::new(r"(?m)^-\s+\|").unwrap();
}

/// Transform Logseq content to Quartz-compatible format
pub fn transform(content: &str, page_index: &PageIndex) -> String {
    let mut result = content.to_string();

    // Remove system properties (not user data)
    result = SYSTEM_PROPS_RE.replace_all(&result, "").to_string();

    // Convert user inline properties to readable format: key:: value → - **Key:** value
    result = USER_PROPS_RE
        .replace_all(&result, |caps: &Captures| {
            let indent = &caps[1];
            let key = &caps[2];
            let value = &caps[3];
            // Convert key-with-dashes to Title Case
            let formatted_key: String = key
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        Some(first) => first.to_uppercase().chain(chars).collect(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!("{}- **{}:** {}", indent, formatted_key, value)
        })
        .to_string();

    // Strip Logseq image size attributes
    result = IMAGE_SIZE_RE.replace_all(&result, "").to_string();

    // Remove empty bullet lines
    result = EMPTY_BULLET_RE.replace_all(&result, "").to_string();

    // Fix tables - remove list marker from first cell
    result = TABLE_FIRST_CELL_RE.replace_all(&result, "|").to_string();

    // Escape $ in wikilinks
    result = WIKILINK_DOLLAR_RE
        .replace_all(&result, |caps: &Captures| {
            let inner = &caps[1];
            format!("[[{}]]", inner.replace('$', "\\$"))
        })
        .to_string();

    // Escape standalone $ tokens (preserve char before $, skip if already escaped)
    result = DOLLAR_TOKEN_RE
        .replace_all(&result, |caps: &Captures| {
            let prefix = &caps[1]; // char before $ or empty at start
            let token = &caps[2];  // the TOKEN part
            format!("{}\\${}", prefix, token)
        })
        .to_string();

    // Convert embeds
    result = EMBED_RE.replace_all(&result, "![[$1]]").to_string();

    // Process wikilinks - remove pages/ prefix if present (pages are now at content root)
    result = WIKILINK_RE
        .replace_all(&result, |caps: &Captures| {
            let embed = caps.get(1).map_or("", |m| m.as_str());
            let link = &caps[2];
            let alias = caps.get(3).map_or("", |m| m.as_str());

            // Remove pages/ prefix since pages are now at content root
            let clean_link = if link.starts_with("pages/") {
                &link[6..] // Remove "pages/" prefix
            } else {
                link
            };

            format!("{}[[{}{}]]", embed, clean_link, alias)
        })
        .to_string();

    // Execute queries and replace with results
    result = QUERY_RE
        .replace_all(&result, |caps: &Captures| {
            let full_match = &caps[0];
            let list_marker = caps.get(1).map_or("", |m| m.as_str());
            let query_str = full_match.trim_start_matches('-').trim();

            let results = query::execute(query_str, page_index);
            let output = query::results_to_markdown(&results, query_str);

            if list_marker.is_empty() {
                output
            } else {
                format!("{}{}", list_marker, output.replace('\n', &format!("\n{}", list_marker)))
            }
        })
        .to_string();

    // Block embed placeholder
    result = BLOCK_EMBED_RE
        .replace_all(&result, "*Block embed - view in Logseq*")
        .to_string();

    // Block references
    result = BLOCK_REF_RE
        .replace_all(&result, "[→ block](#^$1)")
        .to_string();

    // Media embeds
    result = YOUTUBE_RE.replace_all(&result, "![$1]($1)").to_string();
    result = VIDEO_RE.replace_all(&result, "![$1]($1)").to_string();
    result = PDF_RE.replace_all(&result, "![$1]($1)").to_string();

    // Renderer placeholder
    result = RENDERER_RE.replace_all(&result, "`[renderer]`").to_string();

    // Hiccup/EDN syntax - convert to markdown
    result = convert_hiccup_to_markdown(&result);

    // Cloze to highlight
    result = CLOZE_RE.replace_all(&result, "==$1==").to_string();

    // Task markers
    result = DONE_RE.replace_all(&result, "$1- [x] ").to_string();
    result = TODO_RE.replace_all(&result, "$1- [ ] ").to_string();
    result = NOW_RE.replace_all(&result, "$1- [ ] 🔄 ").to_string();
    result = DOING_RE.replace_all(&result, "$1- [ ] 🔄 ").to_string();
    result = LATER_RE.replace_all(&result, "$1- [ ] 📅 ").to_string();
    result = WAITING_RE.replace_all(&result, "$1- [ ] ⏳ ").to_string();
    result = CANCELLED_RE.replace_all(&result, "$1- [x] ❌ ").to_string();

    // Priority markers
    result = PRIORITY_A_RE.replace_all(&result, "🔴").to_string();
    result = PRIORITY_B_RE.replace_all(&result, "🟡").to_string();
    result = PRIORITY_C_RE.replace_all(&result, "🟢").to_string();

    // Schedule/deadline
    result = SCHEDULED_RE
        .replace_all(&result, "📅 Scheduled: $1")
        .to_string();
    result = DEADLINE_RE
        .replace_all(&result, "⏰ Deadline: $1")
        .to_string();

    result
}

/// Convert Logseq hiccup syntax to markdown
fn convert_hiccup_to_markdown(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if line contains hiccup (starts with [: after optional list marker)
        if trimmed.starts_with("[:") || trimmed.starts_with("- [:") {
            let hiccup = if trimmed.starts_with("- ") {
                &trimmed[2..]
            } else {
                trimmed
            };

            // Convert hiccup to markdown
            let markdown = parse_hiccup_to_markdown(hiccup);
            result.push_str(&markdown);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Remove trailing newline if original didn't have one
    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Parse hiccup structure and convert to markdown (preserving element order)
fn parse_hiccup_to_markdown(hiccup: &str) -> String {
    let mut markdown = String::new();

    // Collect all elements with their positions
    let mut elements: Vec<(usize, String)> = Vec::new();

    // Find h2 headers with positions
    for caps in HICCUP_H2_RE.captures_iter(hiccup) {
        if let (Some(m), Some(text)) = (caps.get(0), caps.get(1)) {
            elements.push((m.start(), format!("## {}\n", text.as_str())));
        }
    }

    // Find h3 headers with positions
    for caps in HICCUP_H3_RE.captures_iter(hiccup) {
        if let (Some(m), Some(text)) = (caps.get(0), caps.get(1)) {
            elements.push((m.start(), format!("### {}\n", text.as_str())));
        }
    }

    // Find [:ul ...] blocks and extract their [:li ...] items
    // We need to find each [:ul and its corresponding [:li items
    let ul_starts: Vec<usize> = hiccup.match_indices("[:ul").map(|(i, _)| i).collect();

    for ul_start in ul_starts {
        // Find the extent of this [:ul block by counting brackets
        let ul_slice = &hiccup[ul_start..];
        let mut bracket_count = 0;
        let mut ul_end = 0;

        for (i, c) in ul_slice.char_indices() {
            match c {
                '[' => bracket_count += 1,
                ']' => {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        ul_end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        if ul_end > 0 {
            let ul_content = &ul_slice[..ul_end];
            // Extract all [:li "..."] from this ul block
            let mut list_md = String::new();
            for caps in HICCUP_LI_RE.captures_iter(ul_content) {
                if let Some(text) = caps.get(1) {
                    list_md.push_str(&format!("- {}\n", text.as_str()));
                }
            }
            if !list_md.is_empty() {
                elements.push((ul_start, list_md));
            }
        }
    }

    // Sort by position to maintain order
    elements.sort_by_key(|(pos, _)| *pos);

    // Build markdown output
    for (_, content) in elements {
        markdown.push_str(&content);
    }

    // If we couldn't extract anything meaningful, show as info callout
    if markdown.is_empty() {
        // Extract any quoted strings as fallback
        let mut texts: Vec<String> = Vec::new();
        for caps in HICCUP_TEXT_RE.captures_iter(hiccup) {
            if let Some(text) = caps.get(1) {
                texts.push(text.as_str().to_string());
            }
        }

        if !texts.is_empty() {
            markdown.push_str("> [!info] Dynamic Content\n");
            for text in texts {
                markdown.push_str(&format!("> {}\n", text));
            }
        } else {
            markdown.push_str("> [!note] Dynamic content - view in Logseq\n");
        }
    }

    markdown
}
