use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::page::PageIndex;

lazy_static! {
    // Logseq system properties to remove completely (not user data)
    // Note: query-* properties (query-table, query-properties, query-sort-by, query-sort-desc)
    // are handled by query processing, not removed here
    static ref SYSTEM_PROPS_RE: Regex = Regex::new(
        r"(?m)^(\s*)(?:-\s*)?(collapsed|logseq\.order-list-type|id):: .+$"
    ).unwrap();

    // LOGBOOK blocks (time tracking) - remove lines containing :LOGBOOK:, CLOCK:, :END:
    static ref LOGBOOK_RE: Regex = Regex::new(r"(?m)^\s*(:LOGBOOK:|CLOCK:.*|:END:)\s*$").unwrap();

    // User inline properties to convert to readable format (key:: value → **Key:** value)
    static ref USER_PROPS_RE: Regex = Regex::new(
        r"(?m)^(\s*)(?:-\s*)?([\w-]+):: (.+)$"
    ).unwrap();

    // Logseq image size attributes {:height N, :width N}
    static ref IMAGE_SIZE_RE: Regex = Regex::new(r"\{:height\s+\d+,?\s*:width\s+\d+\}").unwrap();

    // Empty bullet lines (just "- " or "-" with optional whitespace)
    static ref EMPTY_BULLET_RE: Regex = Regex::new(r"(?m)^(\s*)-\s*$").unwrap();

    // Standalone $ tokens (matches $TOKEN patterns)
    static ref DOLLAR_TOKEN_RE: Regex = Regex::new(r"(^|[^\\])\$([A-Z][A-Z0-9]*)").unwrap();

    // Currency patterns like $100, $50,000, $1.99, $10k, $7M ($ followed by digits, optional suffix)
    static ref DOLLAR_CURRENCY_RE: Regex = Regex::new(r"(^|[^\\])\$(\d[\d,.]*[kKmMbB]?)").unwrap();

    // Markdown link with wikilink URL: [text]([[Page]]) -> [text](Page)
    static ref MD_LINK_WIKILINK_RE: Regex = Regex::new(r"\[([^\]]+)\]\(\[\[([^\]]+)\]\]\)").unwrap();

    // Embed syntax
    static ref EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\[\[([^\]]+)\]\]\s*\}\}").unwrap();

    // Block embed
    static ref BLOCK_EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\(\(([^)]+)\)\)\s*\}\}").unwrap();

    // Block reference
    static ref BLOCK_REF_RE: Regex = Regex::new(r"\(\(([a-f0-9-]{36})\)\)").unwrap();

    // Query syntax - captures indentation and optional list marker
    static ref QUERY_RE: Regex = Regex::new(r"(?m)^(\s*)(-\s*)?\{\{query[^\}]*\}\}").unwrap();

    // YouTube/video/pdf embeds
    static ref YOUTUBE_RE: Regex = Regex::new(r"\{\{youtube\s+([^\}]+)\}\}").unwrap();
    static ref VIDEO_RE: Regex = Regex::new(r"\{\{video\s+([^\}]+)\}\}").unwrap();
    static ref PDF_RE: Regex = Regex::new(r"\{\{pdf\s+([^\}]+)\}\}").unwrap();
    // PDF files embedded using image syntax ![name.pdf](path.pdf) or ![](path.pdf)
    static ref IMAGE_PDF_RE: Regex = Regex::new(r"!\[[^\]]*\]\(([^\)]+\.pdf)\)").unwrap();

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

}

/// Transform Logseq content to Quartz-compatible format
pub fn transform(content: &str, page_index: &PageIndex) -> String {
    let mut result = content.to_string();

    // Remove system properties (not user data)
    result = SYSTEM_PROPS_RE.replace_all(&result, "").to_string();

    // Remove LOGBOOK blocks (time tracking)
    result = LOGBOOK_RE.replace_all(&result, "").to_string();

    // Execute queries FIRST (before user props transformation destroys query options)
    result = process_queries_with_options(&result, page_index);

    // Convert user inline properties to readable format: key:: value → - **Key:** value
    // Skip query-* properties as they've been consumed by query processing
    result = USER_PROPS_RE
        .replace_all(&result, |caps: &Captures| {
            let indent = &caps[1];
            let key = &caps[2];
            let value = &caps[3];

            // Skip query options (already processed)
            if key.starts_with("query-") {
                return String::new();
            }

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

    // Fix tables - extract from bullet points and format as proper markdown tables
    result = fix_tables(&result);

    // Escape $ signs for LaTeX compatibility, but NOT inside wikilinks
    // Strategy: protect wikilinks with placeholders, escape $, restore wikilinks
    result = escape_dollars_outside_wikilinks(&result);

    // Convert embeds
    result = EMBED_RE.replace_all(&result, "![[$1]]").to_string();

    // Convert markdown links with wikilink URLs: [text]([[Page]]) -> [text](Page)
    result = MD_LINK_WIKILINK_RE.replace_all(&result, "[$1]($2)").to_string();

    // Process wikilinks - remove pages/ prefix and apply prefix matching for broken links
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

            // Try to find a matching page using prefix matching
            // e.g., "visit us" should match "visit" if "visit" exists but "visit us" doesn't
            let final_link = find_best_page_match(clean_link, page_index);

            // If we found a different page and there's no alias, add the original as alias
            if final_link != clean_link && alias.is_empty() {
                format!("{}[[{}|{}]]", embed, final_link, clean_link)
            } else {
                format!("{}[[{}{}]]", embed, final_link, alias)
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
    // PDF embed - use iframe for embedding
    result = PDF_RE.replace_all(&result, r#"<iframe src="$1" width="100%" height="600px" style="border: 1px solid #333; border-radius: 4px;"></iframe>"#).to_string();
    // PDF embedded as image syntax ![name.pdf](path.pdf) - also convert to iframe
    result = IMAGE_PDF_RE.replace_all(&result, r#"<iframe src="$1" width="100%" height="600px" style="border: 1px solid #333; border-radius: 4px;"></iframe>"#).to_string();

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

/// Escape dollar signs for LaTeX compatibility, but NOT inside wikilinks
/// Wikilinks like [[$BOOT]] must keep $ unescaped to match page names
fn escape_dollars_outside_wikilinks(content: &str) -> String {
    // Regex to match wikilinks (including embeds like ![[...]])
    lazy_static::lazy_static! {
        static ref WIKILINK_PLACEHOLDER_RE: Regex = Regex::new(r"(!?\[\[[^\]]+\]\])").unwrap();
    }

    // Step 1: Extract wikilinks and replace with placeholders
    let mut placeholders: Vec<String> = Vec::new();
    let protected = WIKILINK_PLACEHOLDER_RE.replace_all(content, |caps: &Captures| {
        let wikilink = caps[1].to_string();
        let placeholder = format!("\x00WIKILINK{}\x00", placeholders.len());
        placeholders.push(wikilink);
        placeholder
    }).to_string();

    // Step 2: Escape $ tokens (like $HOME, $BOOT) - uppercase tokens
    let escaped = DOLLAR_TOKEN_RE
        .replace_all(&protected, |caps: &Captures| {
            let prefix = &caps[1];
            let token = &caps[2];
            format!("{}\\${}", prefix, token)
        })
        .to_string();

    // Step 3: Escape currency patterns (like $100, $10k)
    let escaped = DOLLAR_CURRENCY_RE
        .replace_all(&escaped, |caps: &Captures| {
            let prefix = &caps[1];
            let amount = &caps[2];
            format!("{}\\${}", prefix, amount)
        })
        .to_string();

    // Step 4: Restore wikilinks from placeholders
    let mut result = escaped;
    for (i, wikilink) in placeholders.iter().enumerate() {
        let placeholder = format!("\x00WIKILINK{}\x00", i);
        result = result.replace(&placeholder, wikilink);
    }

    result
}

/// Process queries with context-aware options (query-properties::, query-sort-by::, etc.)
fn process_queries_with_options(content: &str, page_index: &crate::page::PageIndex) -> String {
    use crate::query;

    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines: Vec<String> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Check if this line contains a query
        if let Some(caps) = QUERY_RE.captures(line) {
            let full_match = &caps[0];
            let indent = caps.get(1).map_or("", |m| m.as_str());
            let list_marker = caps.get(2).map_or("", |m| m.as_str());
            let full_prefix = format!("{}{}", indent, list_marker);
            let query_str = full_match.trim().trim_start_matches('-').trim();

            // Look at previous lines for query options (within the same block)
            let mut context = String::new();
            let mut j = result_lines.len();

            while j > 0 {
                j -= 1;
                let prev_line = &result_lines[j];
                // Stop if we hit an empty line or a line that doesn't look like a property
                if prev_line.trim().is_empty() {
                    break;
                }
                // Check if it's a query option line
                if prev_line.contains("query-properties::")
                    || prev_line.contains("query-sort-by::")
                    || prev_line.contains("query-sort-desc::")
                    || prev_line.contains("query-table::")
                {
                    context = format!("{}\n{}", prev_line, context);
                } else {
                    break;
                }
            }

            // Parse options from context
            let options = query::parse_query_options(&context);

            // Execute query and render results
            let results = query::execute(query_str, page_index);
            let output = query::results_to_markdown_with_options(&results, query_str, &options);

            // Format output with proper indentation
            let formatted_output = if output.contains('|') && output.contains("---") {
                // Table output - needs blank line before for markdown to recognize it
                // Tables should NOT have list markers, just indentation
                let table_lines: Vec<_> = output
                    .lines()
                    .map(|line| format!("{}{}", indent, line))
                    .collect();
                // Add blank line before table for proper markdown parsing
                format!("\n{}", table_lines.join("\n"))
            } else {
                // List output - add full prefix (indent + list marker) to each line
                // If no list marker on the query, use "- " as default
                let effective_prefix = if list_marker.is_empty() {
                    format!("{}- ", indent)
                } else {
                    full_prefix.clone()
                };
                output
                    .lines()
                    .map(|line| {
                        // Strip the "- " prefix from results, then add proper indentation
                        let content = line.strip_prefix("- ").unwrap_or(line);
                        format!("{}{}", effective_prefix, content)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            };

            // Replace the query line with output - push each line separately
            for output_line in formatted_output.lines() {
                result_lines.push(output_line.to_string());
            }
        } else {
            result_lines.push(line.to_string());
        }

        i += 1;
    }

    result_lines.join("\n")
}

/// Convert Logseq hiccup syntax to HTML
fn convert_hiccup_to_markdown(content: &str) -> String {
    let mut result = String::new();
    let mut in_multiline_hiccup = false;
    let mut hiccup_buffer = String::new();
    let mut hiccup_indent = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if this starts a hiccup block
        if !in_multiline_hiccup && (trimmed.starts_with("[:") || trimmed.starts_with("- [:")) {
            let hiccup_start = if trimmed.starts_with("- ") {
                hiccup_indent = line.chars().take_while(|c| c.is_whitespace()).collect();
                &trimmed[2..]
            } else {
                hiccup_indent = String::new();
                trimmed
            };

            // Check if hiccup is complete on one line (balanced brackets)
            let mut bracket_count = 0;
            for c in hiccup_start.chars() {
                match c {
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => {}
                }
            }

            if bracket_count == 0 {
                // Single-line hiccup
                let html = hiccup_to_html(hiccup_start);
                result.push_str(&hiccup_indent);
                // Don't add list marker for block elements
                if !is_block_element(&html) {
                    result.push_str("- ");
                }
                result.push_str(&html);
                result.push('\n');
            } else {
                // Multi-line hiccup starts
                in_multiline_hiccup = true;
                hiccup_buffer = hiccup_start.to_string();
            }
        } else if in_multiline_hiccup {
            // Continue collecting multi-line hiccup
            hiccup_buffer.push(' ');
            hiccup_buffer.push_str(trimmed);

            // Check if hiccup is now complete
            let mut bracket_count = 0;
            for c in hiccup_buffer.chars() {
                match c {
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => {}
                }
            }

            if bracket_count == 0 {
                in_multiline_hiccup = false;
                let html = hiccup_to_html(&hiccup_buffer);
                result.push_str(&hiccup_indent);
                // Don't add list marker for block elements
                if !is_block_element(&html) {
                    result.push_str("- ");
                }
                result.push_str(&html);
                result.push('\n');
                hiccup_buffer.clear();
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Handle unclosed hiccup
    if !hiccup_buffer.is_empty() {
        result.push_str(&hiccup_indent);
        result.push_str("- ");
        result.push_str(&hiccup_to_html(&hiccup_buffer));
        result.push('\n');
    }

    // Remove trailing newline if original didn't have one
    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Check if HTML string is a block-level element (shouldn't be wrapped in list)
fn is_block_element(html: &str) -> bool {
    html.starts_with("<ul")
        || html.starts_with("<ol")
        || html.starts_with("<div")
        || html.starts_with("<table")
        || html.starts_with("<h1")
        || html.starts_with("<h2")
        || html.starts_with("<h3")
        || html.starts_with("<h4")
        || html.starts_with("<blockquote")
        || html.starts_with("<pre")
}

/// Convert hiccup syntax to HTML
fn hiccup_to_html(hiccup: &str) -> String {
    let hiccup = hiccup.trim();

    // Parse the tag and check for attributes
    if !hiccup.starts_with("[:") {
        return format!("`{}`", hiccup);
    }

    // Extract tag name
    let after_bracket = &hiccup[2..];
    let tag_end = after_bracket.find(|c: char| c.is_whitespace() || c == ']').unwrap_or(after_bracket.len());
    let tag = &after_bracket[..tag_end];

    // Get the rest (attributes + content)
    let rest = &after_bracket[tag_end..].trim_end_matches(']').trim();

    // Check for attributes map {:key "value" ...}
    let (attrs_html, content_start) = if rest.starts_with('{') {
        // Find matching closing brace
        let mut brace_count = 0;
        let mut attr_end = 0;
        for (i, c) in rest.char_indices() {
            match c {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        attr_end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        let attr_str = &rest[1..attr_end - 1]; // Remove { }
        let attrs = parse_hiccup_attrs(attr_str);
        (attrs, &rest[attr_end..])
    } else {
        (String::new(), *rest)
    };

    let content = parse_hiccup_content(content_start);

    // Handle special tags
    match tag {
        "ul" | "ol" => {
            let mut items = Vec::new();
            for caps in HICCUP_LI_RE.captures_iter(hiccup) {
                if let Some(text) = caps.get(1) {
                    items.push(format!("<li>{}</li>", text.as_str()));
                }
            }
            if !items.is_empty() {
                return format!("<{}{}>{}</{}>", tag, attrs_html, items.join(""), tag);
            }
        }
        _ => {}
    }

    // Generate HTML
    if content.is_empty() && attrs_html.is_empty() {
        format!("<{}/>", tag)
    } else {
        format!("<{}{}>{}</{}>", tag, attrs_html, content, tag)
    }
}

/// Parse hiccup attributes {:key "value" :key2 "value2"}
fn parse_hiccup_attrs(attrs: &str) -> String {
    lazy_static::lazy_static! {
        static ref ATTR_RE: Regex = Regex::new(r#":(\w+)\s+"([^"]+)""#).unwrap();
    }

    let mut result = String::new();
    for caps in ATTR_RE.captures_iter(attrs) {
        if let (Some(key), Some(value)) = (caps.get(1), caps.get(2)) {
            result.push(' ');
            result.push_str(key.as_str());
            result.push_str("=\"");
            result.push_str(value.as_str());
            result.push('"');
        }
    }
    result
}

/// Parse content inside hiccup element (handles nested elements and strings)
fn parse_hiccup_content(content: &str) -> String {
    let content = content.trim();
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = content.chars().collect();

    while i < chars.len() {
        if chars[i] == '"' {
            // Parse quoted string
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            result.push_str(&chars[start..i].iter().collect::<String>());
            i += 1; // skip closing quote
        } else if chars[i] == '[' && i + 1 < chars.len() && chars[i + 1] == ':' {
            // Parse nested hiccup element
            let start = i;
            let mut bracket_count = 0;
            while i < chars.len() {
                match chars[i] {
                    '[' => bracket_count += 1,
                    ']' => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            i += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let nested: String = chars[start..i].iter().collect();
            result.push_str(&hiccup_to_html(&nested));
        } else if !chars[i].is_whitespace() {
            // Skip other characters
            i += 1;
        } else {
            i += 1;
        }
    }

    result
}

/// Fix tables embedded in Logseq bullet points
/// Logseq tables look like:
/// \t- | col1 | col2 |
/// \t  |------|------|
/// \t  | val1 | val2 |
/// This function adds separator rows if missing while preserving document structure
fn fix_tables(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Check if this line contains a table row (has | and looks like table syntax)
        // A table row starts with optional whitespace, optional "- ", then "|"
        let trimmed = line.trim();
        let after_bullet = trimmed.trim_start_matches('-').trim_start();

        if after_bullet.starts_with('|') && after_bullet.matches('|').count() >= 2 {
            // Found a table start - collect all table rows while preserving structure
            let mut table_lines: Vec<String> = Vec::new();
            let mut table_contents: Vec<String> = Vec::new(); // Just the | ... | parts

            // Get the indentation prefix (everything before the | part)
            let first_line_prefix = get_line_prefix(line);
            table_lines.push(line.to_string());
            table_contents.push(after_bullet.to_string());
            i += 1;

            // Collect continuation lines
            while i < lines.len() {
                let next_line = lines[i];
                let next_trimmed = next_line.trim();

                // Check if this is a table continuation (indented line with |)
                if next_trimmed.starts_with('|') {
                    table_lines.push(next_line.to_string());
                    table_contents.push(next_trimmed.to_string());
                    i += 1;
                } else {
                    break;
                }
            }

            // Check if table has a valid separator row with correct column count
            let header_col_count = table_contents[0].matches('|').count().saturating_sub(1);
            let has_valid_separator = table_contents.iter().any(|line| {
                if is_separator_row(line) {
                    // Separator must have same column count as header
                    let sep_col_count = line.matches('|').count().saturating_sub(1);
                    sep_col_count == header_col_count
                } else {
                    false
                }
            });

            // Output table with separator if needed
            // First row (header)
            result.push(table_lines[0].clone());

            // Add separator after first row if missing or invalid
            if !has_valid_separator && table_contents.len() > 1 {
                let col_count = header_col_count;
                if col_count > 0 {
                    // Use same indentation as continuation lines (without the "- ")
                    let continuation_prefix = get_continuation_prefix(&first_line_prefix);
                    let separator = format!("{}|{}|", continuation_prefix, vec!["---"; col_count].join("|"));
                    result.push(separator);
                }
            }

            // Add remaining rows, skipping malformed separator rows
            for (idx, table_line) in table_lines.iter().skip(1).enumerate() {
                let content = &table_contents[idx + 1];
                // Skip separator rows with wrong column count (malformed)
                if is_separator_row(content) {
                    let sep_col_count = content.matches('|').count().saturating_sub(1);
                    if sep_col_count != header_col_count {
                        continue; // Skip malformed separator
                    }
                }
                result.push(table_line.clone());
            }
        } else {
            result.push(line.to_string());
            i += 1;
        }
    }

    result.join("\n")
}

/// Check if a line is a markdown table separator row (only |, -, :, spaces)
fn is_separator_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|')
        && trimmed.ends_with('|')
        && trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
        && trimmed.contains('-')
}

/// Get the prefix (indentation + bullet) from a line
fn get_line_prefix(line: &str) -> String {
    // Find where the | starts
    if let Some(pos) = line.find('|') {
        line[..pos].to_string()
    } else {
        String::new()
    }
}

/// Get continuation line prefix from first line prefix
/// e.g., "\t- " becomes "\t  " (replace "- " with "  ")
fn get_continuation_prefix(first_prefix: &str) -> String {
    // Replace the last "- " with "  " for continuation lines
    if let Some(pos) = first_prefix.rfind("- ") {
        let mut result = first_prefix.to_string();
        result.replace_range(pos..pos+2, "  ");
        result
    } else if first_prefix.ends_with('-') {
        // Handle case where it's just "-" without space
        let mut result = first_prefix.to_string();
        result.pop();
        result.push(' ');
        result
    } else {
        first_prefix.to_string()
    }
}

/// Find the best matching page for a wikilink using alias and prefix matching
/// Handles:
/// 1. Exact page name match
/// 2. Exact alias match (e.g., "cv/districts" matches page with alias "cv/districts")
/// 3. Namespace alias expansion (e.g., "cv/districts" → "cyber valley/districts" if "cv" is alias for "cyber valley")
/// 4. Prefix matching (e.g., "visit us" matches "visit" if "visit us" doesn't exist)
fn find_best_page_match<'a>(link: &'a str, page_index: &[crate::page::Page]) -> &'a str {
    let link_lower = link.to_lowercase();
    let link_normalized = link_lower.replace(' ', "-").replace('_', "-");

    // 1. Check for exact page name match
    for page in page_index {
        let page_name = page.name.to_lowercase();
        let page_normalized = page_name.replace(' ', "-").replace('_', "-");

        if page_name == link_lower || page_normalized == link_normalized {
            return link; // Exact match, return original
        }
    }

    // 2. Check for exact alias match
    for page in page_index {
        for alias in &page.aliases {
            let alias_lower = alias.to_lowercase();
            let alias_normalized = alias_lower.replace(' ', "-").replace('_', "-");

            if alias_lower == link_lower || alias_normalized == link_normalized {
                // Found alias match - return the page name
                return Box::leak(page.name.clone().into_boxed_str());
            }
        }
    }

    // 3. Namespace alias expansion: if link is "prefix/suffix", check if "prefix" is an alias
    if link.contains('/') {
        let parts: Vec<&str> = link.splitn(2, '/').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            let prefix_lower = prefix.to_lowercase();

            // Find what page "prefix" is an alias for
            for page in page_index {
                for alias in &page.aliases {
                    if alias.to_lowercase() == prefix_lower {
                        // Found: prefix is alias for page.name
                        // Now look for "page.name/suffix"
                        let expanded_link = format!("{}/{}", page.name, suffix);
                        let expanded_lower = expanded_link.to_lowercase();

                        // Check if expanded link matches any page
                        for target_page in page_index {
                            let target_name = target_page.name.to_lowercase();
                            if target_name == expanded_lower {
                                return Box::leak(target_page.name.clone().into_boxed_str());
                            }
                            // Also check aliases of target page
                            for target_alias in &target_page.aliases {
                                if target_alias.to_lowercase() == link_lower {
                                    return Box::leak(target_page.name.clone().into_boxed_str());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. Prefix matching: "visit us" matches "visit" if "visit" exists
    let mut best_match: Option<&str> = None;
    let mut best_len = 0;

    let link_words = link_lower.replace('-', " ").replace('_', " ");

    for page in page_index {
        let page_name = page.name.to_lowercase();
        let page_words = page_name.replace('-', " ").replace('_', " ");

        // Check if link starts with page name followed by a space
        if link_words.len() > page_words.len()
            && link_words.starts_with(&page_words)
            && link_words.chars().nth(page_words.len()) == Some(' ')
        {
            if page_words.len() > best_len {
                best_len = page_words.len();
                best_match = Some(&page.name);
            }
        }
    }

    // Return the best match or original link
    if let Some(matched) = best_match {
        Box::leak(matched.to_string().into_boxed_str())
    } else {
        link
    }
}
