use lazy_static::lazy_static;
use regex::Regex;

use crate::page::{Page, PageIndex};

lazy_static! {
    // Query patterns - allow optional whitespace before closing paren
    static ref PAGE_TAGS_RE: Regex = Regex::new(r"(?i)^\(page-tags\s+\[\[([^\]]+)\]\]\s*\)$").unwrap();
    static ref PAGE_RE: Regex = Regex::new(r"(?i)^\(page\s+\[\[([^\]]+)\]\]\s*\)$").unwrap();
    static ref NAMESPACE_RE: Regex = Regex::new(r"(?i)^\(namespace\s+\[\[([^\]]+)\]\]\s*\)$").unwrap();
    static ref PROPERTY_RE: Regex = Regex::new(r#"(?i)^\((?:page-)?property\s+:?(\w+[-\w]*)(?:\s+(?:"([^"]+)"|(\S+)))?\s*\)$"#).unwrap();
    // Matches (task STATE) or (task STATE1 STATE2 ...)
    static ref TASK_RE: Regex = Regex::new(r"(?i)^\(task\s+((?:TODO|DONE|NOW|DOING|LATER|WAITING|CANCELLED)(?:\s+(?:TODO|DONE|NOW|DOING|LATER|WAITING|CANCELLED))*)\s*\)$").unwrap();
    static ref PAGE_REF_RE: Regex = Regex::new(r"^\[\[([^\]]+)\]\]$").unwrap();
    static ref TEXT_SEARCH_RE: Regex = Regex::new(r#"^"([^"]+)"$"#).unwrap();

    // New query patterns - allow optional whitespace before closing paren
    static ref PRIORITY_RE: Regex = Regex::new(r"(?i)^\(priority\s+([abc])\s*\)$").unwrap();
    static ref BETWEEN_RE: Regex = Regex::new(r"(?i)^\(between\s+\[\[([^\]]+)\]\]\s+\[\[([^\]]+)\]\]\s*\)$").unwrap();
    static ref SORT_BY_RE: Regex = Regex::new(r"(?i)^\(sort-by\s+:?(\w+[-\w]*)\s*(asc|desc)?\s*\)$").unwrap();
    static ref ALL_PAGE_TAGS_RE: Regex = Regex::new(r"(?i)^\(all-page-tags\s*\)$").unwrap();

    // Query options (inline properties)
    static ref QUERY_PROPS_RE: Regex = Regex::new(r"query-properties::\s*\[:?([^\]]+)\]").unwrap();
    static ref QUERY_SORT_BY_RE: Regex = Regex::new(r"query-sort-by::\s*:?(\S+)").unwrap();
    static ref QUERY_SORT_DESC_RE: Regex = Regex::new(r"query-sort-desc::\s*(true|false)").unwrap();
    static ref QUERY_TABLE_RE: Regex = Regex::new(r"query-table::\s*(true|false)").unwrap();
}

/// Execute a Logseq query and return matching pages
pub fn execute<'a>(query_str: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    // Extract the query expression from {{query ...}}
    let expr = query_str
        .trim()
        .trim_start_matches("{{query")
        .trim_end_matches("}}")
        .trim();

    execute_expr(expr, index)
}

fn execute_expr<'a>(expr: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    // Normalize whitespace: trim and collapse multiple spaces
    let expr = expr.trim();

    // Helper to extract inner content from (keyword ...) with flexible whitespace
    fn extract_inner<'b>(expr: &'b str, keyword: &str) -> Option<&'b str> {
        let lower = expr.to_lowercase();
        if lower.starts_with(&format!("({}", keyword)) && expr.ends_with(')') {
            // Find where the keyword ends and content begins
            let after_paren = &expr[1..]; // skip '('
            let after_keyword = after_paren.strip_prefix(keyword)
                .or_else(|| after_paren.strip_prefix(&keyword.to_uppercase()))?;
            let inner = after_keyword.trim_start(); // skip whitespace after keyword
            // Remove trailing ')'
            if inner.ends_with(')') {
                return Some(&inner[..inner.len() - 1]);
            }
        }
        None
    }

    // Handle (and ...)
    if let Some(inner) = extract_inner(expr, "and") {
        return execute_and(inner, index);
    }

    // Handle (or ...)
    if let Some(inner) = extract_inner(expr, "or") {
        return execute_or(inner, index);
    }

    // Handle (not ...)
    if let Some(inner) = extract_inner(expr, "not") {
        let excluded = execute_expr(inner, index);
        let excluded_names: std::collections::HashSet<_> =
            excluded.iter().map(|p| &p.name).collect();
        return index.iter().filter(|p| !excluded_names.contains(&p.name)).collect();
    }

    // Handle (task STATE) or (task STATE1 STATE2 ...)
    if let Some(caps) = TASK_RE.captures(expr) {
        let states_str = caps.get(1).unwrap().as_str().to_uppercase();
        let states: Vec<&str> = states_str.split_whitespace().collect();
        return index
            .iter()
            .filter(|p| {
                states.iter().any(|state| {
                    p.content.contains(&format!("- {} ", state))
                        || p.content.contains(&format!("\n{} ", state))
                })
            })
            .collect();
    }

    // Handle (priority a/b/c)
    if let Some(caps) = PRIORITY_RE.captures(expr) {
        let priority = caps.get(1).unwrap().as_str().to_uppercase();
        let pattern = format!("[#{}]", priority);
        return index
            .iter()
            .filter(|p| p.content.contains(&pattern))
            .collect();
    }

    // Handle (between [[date1]] [[date2]]) - for journal pages
    if let Some(caps) = BETWEEN_RE.captures(expr) {
        let start_date = caps.get(1).unwrap().as_str();
        let end_date = caps.get(2).unwrap().as_str();
        if let (Some(start), Some(end)) = (parse_date(start_date), parse_date(end_date)) {
            return index
                .iter()
                .filter(|p| {
                    // Check if page name looks like a date (strip journals/ prefix if present)
                    let name = p.name.strip_prefix("journals/").unwrap_or(&p.name);
                    if let Some(page_date) = parse_date(name) {
                        page_date >= start && page_date <= end
                    } else {
                        false
                    }
                })
                .collect();
        }
    }

    // Handle (all-page-tags) - returns all unique tags as virtual results
    if ALL_PAGE_TAGS_RE.is_match(expr) {
        // This is a special case - we return pages that have the tag names
        // For now, collect all unique tags and return pages tagged with them
        let mut all_tags: std::collections::HashSet<String> = std::collections::HashSet::new();
        for page in index.iter() {
            for tag in &page.tags {
                all_tags.insert(tag.clone());
            }
        }
        // Return pages that match tag names (if they exist as pages)
        return index
            .iter()
            .filter(|p| all_tags.contains(&p.name_lower))
            .collect();
    }

    // Handle (page [[name]])
    if let Some(caps) = PAGE_RE.captures(expr) {
        let page_name = caps.get(1).unwrap().as_str().to_lowercase();
        // Strip pages/ prefix if present
        let page_name = page_name.strip_prefix("pages/").unwrap_or(&page_name);
        return index.iter().filter(|p| p.name_lower == page_name).collect();
    }

    // Handle (page-tags [[tag]])
    if let Some(caps) = PAGE_TAGS_RE.captures(expr) {
        let tag = caps.get(1).unwrap().as_str().to_lowercase();
        // Strip pages/ prefix if present
        let tag = tag.strip_prefix("pages/").unwrap_or(&tag);
        return index.iter().filter(|p| p.tags.contains(&tag.to_string())).collect();
    }

    // Handle (namespace [[x]])
    if let Some(caps) = NAMESPACE_RE.captures(expr) {
        let ns = caps.get(1).unwrap().as_str().to_lowercase();
        let ns = ns.strip_prefix("pages/").unwrap_or(&ns);
        return index
            .iter()
            .filter(|p| p.namespace.as_ref().map_or(false, |n| n.to_lowercase() == ns))
            .collect();
    }

    // Handle (property :key value)
    if let Some(caps) = PROPERTY_RE.captures(expr) {
        let key = caps.get(1).unwrap().as_str().to_lowercase().replace('-', "");
        let value = caps
            .get(2)
            .or_else(|| caps.get(3))
            .map(|m| m.as_str().to_lowercase().trim_matches('"').to_string())
            .unwrap_or_default();

        return index
            .iter()
            .filter(|p| {
                let prop_val = p.properties.get(&key).map(|v| v.to_lowercase()).unwrap_or_default();
                if value.is_empty() {
                    !prop_val.is_empty()
                } else {
                    prop_val == value || prop_val.contains(&value)
                }
            })
            .collect();
    }

    // Handle [[page]] reference
    if let Some(caps) = PAGE_REF_RE.captures(expr) {
        let page_name = caps.get(1).unwrap().as_str().to_lowercase();
        let page_name = page_name.strip_prefix("pages/").unwrap_or(&page_name);
        return index
            .iter()
            .filter(|p| {
                p.name_lower == page_name
                    || p.content.to_lowercase().contains(&format!("[[{}]]", page_name))
            })
            .collect();
    }

    // Handle "text" search
    if let Some(caps) = TEXT_SEARCH_RE.captures(expr) {
        let search = caps.get(1).unwrap().as_str().to_lowercase();
        return index
            .iter()
            .filter(|p| p.content.to_lowercase().contains(&search))
            .collect();
    }

    // Plain text search
    if !expr.starts_with('(') && !expr.starts_with('[') {
        let search = expr.to_lowercase().replace(['"', '\''], "");
        if search.len() > 2 {
            return index
                .iter()
                .filter(|p| p.content.to_lowercase().contains(&search))
                .collect();
        }
    }

    Vec::new()
}

fn execute_and<'a>(inner: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    let parts = parse_query_parts(inner);
    if parts.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<&Page> = execute_expr(&parts[0], index);
    for part in parts.iter().skip(1) {
        let matching = execute_expr(part, index);
        let matching_names: std::collections::HashSet<_> =
            matching.iter().map(|p| &p.name).collect();
        result.retain(|p| matching_names.contains(&p.name));
    }
    result
}

fn execute_or<'a>(inner: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    let parts = parse_query_parts(inner);
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for part in parts {
        for page in execute_expr(&part, index) {
            if seen.insert(&page.name) {
                result.push(page);
            }
        }
    }
    result
}

/// Parse query parts handling nested parentheses
fn parse_query_parts(expr: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_brackets = 0;

    for c in expr.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                depth -= 1;
                current.push(c);
                if depth == 0 {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() {
                        parts.push(trimmed);
                    }
                    current.clear();
                }
            }
            '[' => {
                in_brackets += 1;
                current.push(c);
            }
            ']' => {
                in_brackets -= 1;
                current.push(c);
                if depth == 0 && in_brackets == 0 {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
                        parts.push(trimmed);
                    }
                    current.clear();
                }
            }
            ' ' | '\n' | '\t' if depth == 0 && in_brackets == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
                    parts.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
        parts.push(trimmed);
    }

    parts
}

/// Parse date strings in various formats (journal page names, natural language dates)
fn parse_date(date_str: &str) -> Option<chrono::NaiveDate> {
    use chrono::NaiveDate;

    // Try common formats
    // Format: 2024-01-15, 2024_01_15
    let normalized = date_str.replace('_', "-");
    if let Ok(d) = NaiveDate::parse_from_str(&normalized, "%Y-%m-%d") {
        return Some(d);
    }

    // Format: Jan 15th, 2024 or January 15, 2024
    lazy_static::lazy_static! {
        static ref DATE_RE: Regex = Regex::new(
            r"(?i)^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\s+(\d{1,2})(?:st|nd|rd|th)?,?\s+(\d{4})$"
        ).unwrap();
    }

    if let Some(caps) = DATE_RE.captures(date_str) {
        let month_str = caps.get(1).unwrap().as_str().to_lowercase();
        let day: u32 = caps.get(2).unwrap().as_str().parse().ok()?;
        let year: i32 = caps.get(3).unwrap().as_str().parse().ok()?;

        let month = match &month_str[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => return None,
        };

        return NaiveDate::from_ymd_opt(year, month, day);
    }

    None
}

/// Query options parsed from context
#[derive(Default)]
pub struct QueryOptions {
    pub properties: Vec<String>,
    pub sort_by: Option<String>,
    pub sort_desc: bool,
    /// None = default (table), Some(true) = force table, Some(false) = force list
    pub table: Option<bool>,
}

/// Parse query options from surrounding context (the block containing the query)
pub fn parse_query_options(context: &str) -> QueryOptions {
    let mut opts = QueryOptions::default();

    // Parse query-properties:: [:page, :status, :priority]
    if let Some(caps) = QUERY_PROPS_RE.captures(context) {
        let props_str = caps.get(1).unwrap().as_str();
        opts.properties = props_str
            .split([',', ' '])
            .map(|s| s.trim().trim_start_matches(':').to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    // Parse query-sort-by:: :created or query-sort-by:: created
    if let Some(caps) = QUERY_SORT_BY_RE.captures(context) {
        opts.sort_by = Some(caps.get(1).unwrap().as_str().to_string());
    }

    // Parse query-sort-desc:: true/false
    if let Some(caps) = QUERY_SORT_DESC_RE.captures(context) {
        opts.sort_desc = caps.get(1).unwrap().as_str() == "true";
    }

    // Parse query-table:: true/false (explicit override)
    if let Some(caps) = QUERY_TABLE_RE.captures(context) {
        opts.table = Some(caps.get(1).unwrap().as_str() == "true");
    }

    opts
}

/// Convert query results to markdown (with optional table view)
#[allow(dead_code)]
pub fn results_to_markdown(results: &[&Page], query_str: &str) -> String {
    results_to_markdown_with_options(results, query_str, &QueryOptions::default())
}

/// Convert query results to markdown with options support
pub fn results_to_markdown_with_options(
    results: &[&Page],
    query_str: &str,
    options: &QueryOptions,
) -> String {
    if results.is_empty() {
        return format!(
            "> [!info] Query Results\n> No pages match this query.\n> `{}`",
            if query_str.len() > 80 {
                format!("{}...", &query_str[..80])
            } else {
                query_str.to_string()
            }
        );
    }

    // Sort results
    let mut sorted: Vec<_> = results.iter().copied().collect();
    if let Some(ref sort_key) = options.sort_by {
        sorted.sort_by(|a, b| {
            let a_val = get_page_property(a, sort_key);
            let b_val = get_page_property(b, sort_key);
            if options.sort_desc {
                b_val.cmp(&a_val)
            } else {
                a_val.cmp(&b_val)
            }
        });
    } else {
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
    }

    // If properties are specified, render as table with those properties
    if !options.properties.is_empty() {
        return render_table(&sorted, &options.properties);
    }

    // If explicitly disabled with query-table:: false, render as list
    if options.table == Some(false) {
        return render_list(&sorted);
    }

    // Default: auto-detect properties and render as table (like Logseq)
    let auto_props = detect_common_properties(&sorted);
    render_table(&sorted, &auto_props)
}

/// Render results as a markdown list
fn render_list(results: &[&Page]) -> String {
    results
        .iter()
        .map(|p| {
            let icon = p.properties.get("icon").map_or("", |s| s.as_str());
            let title = p
                .properties
                .get("title")
                .map_or(p.name.replace('_', " "), |t| t.clone());
            format!(
                "- [[{}|{}{}]]",
                p.name,
                if icon.is_empty() {
                    String::new()
                } else {
                    format!("{} ", icon)
                },
                title
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Get a property value from a page (supports special properties)
fn get_page_property(page: &Page, key: &str) -> String {
    match key.to_lowercase().as_str() {
        "page" | "name" => page.name.clone(),
        "created" => page.created.clone().unwrap_or_default(),
        "modified" | "updated" => page.modified.clone().unwrap_or_default(),
        "tags" => page.tags.join(", "),
        "namespace" => page.namespace.clone().unwrap_or_default(),
        _ => page
            .properties
            .get(&key.to_lowercase().replace('-', ""))
            .cloned()
            .unwrap_or_default(),
    }
}

/// Render results as a markdown table
fn render_table(results: &[&Page], properties: &[String]) -> String {
    let mut output = String::new();

    // Header row
    output.push('|');
    for prop in properties {
        let header = match prop.to_lowercase().as_str() {
            "page" | "name" => "Page".to_string(),
            _ => {
                // Capitalize first letter
                let mut chars = prop.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        };
        output.push_str(&format!(" {} |", header));
    }
    output.push('\n');

    // Separator row
    output.push('|');
    for _ in properties {
        output.push_str(" --- |");
    }
    output.push('\n');

    // Data rows
    for page in results {
        output.push('|');
        for prop in properties {
            let value = match prop.to_lowercase().as_str() {
                "page" | "name" => {
                    // Use plain wikilink without alias to avoid pipe conflicts in tables
                    format!("[[{}]]", page.name)
                }
                _ => {
                    // Escape any pipes in cell values using HTML entity
                    get_page_property(page, prop).replace('|', "&#124;")
                }
            };
            output.push_str(&format!(" {} |", value));
        }
        output.push('\n');
    }

    output
}

/// Detect common properties from query results for automatic table rendering
fn detect_common_properties(results: &[&Page]) -> Vec<String> {
    // Always include page name first
    let mut props = vec!["page".to_string()];

    // Count property occurrences across results
    let mut prop_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for page in results {
        for key in page.properties.keys() {
            // Skip internal/system properties
            let key_lower = key.to_lowercase();
            if key_lower == "title" || key_lower == "icon" || key_lower == "public"
               || key_lower == "alias" || key_lower == "aliases" {
                continue;
            }
            *prop_counts.entry(key.clone()).or_insert(0) += 1;
        }
        // Count tags if any page has tags
        if !page.tags.is_empty() {
            *prop_counts.entry("tags".to_string()).or_insert(0) += 1;
        }
    }

    // Include properties that appear in at least 30% of results (or at least 1 if few results)
    let threshold = std::cmp::max(1, results.len() / 3);

    // Sort by frequency (most common first), then alphabetically
    let mut prop_list: Vec<_> = prop_counts.into_iter()
        .filter(|(_, count)| *count >= threshold)
        .collect();
    prop_list.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    // Add top properties (limit to 4 columns total including page name)
    for (prop, _) in prop_list.into_iter().take(3) {
        props.push(prop);
    }

    props
}

/// Get all unique tags from the index
#[allow(dead_code)]
pub fn get_all_tags(index: &PageIndex) -> Vec<String> {
    let mut tags: std::collections::HashSet<String> = std::collections::HashSet::new();
    for page in index.iter() {
        for tag in &page.tags {
            tags.insert(tag.clone());
        }
    }
    let mut sorted: Vec<_> = tags.into_iter().collect();
    sorted.sort();
    sorted
}
