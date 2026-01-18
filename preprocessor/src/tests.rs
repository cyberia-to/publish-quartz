#[cfg(test)]
mod path_tests {
    use crate::content;
    use crate::page::PageIndex;

    fn empty_index() -> PageIndex {
        Vec::new()
    }

    #[test]
    fn test_wikilink_preserved() {
        let input = "Check out [[devops]] for more info.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[devops]]"),
            "Wikilinks should be preserved (pages are at content root), got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_namespace_preserved() {
        let input = "See [[terrabyte/garden]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[terrabyte/garden]]"),
            "Namespace pages should be preserved, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_strips_pages_prefix() {
        let input = "See [[pages/cyber]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[cyber]]"),
            "Should strip pages/ prefix (pages are at content root), got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_preserves_journals_prefix() {
        let input = "See [[journals/2025-01-01]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[journals/2025-01-01]]"),
            "Should preserve journals/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_with_alias() {
        let input = "Check [[devops|DevOps Guide]] here.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[devops|DevOps Guide]]"),
            "Should preserve alias, got: {}",
            result
        );
    }

    #[test]
    fn test_embed_converted() {
        let input = "{{embed [[intro]]}}";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("![[intro]]"),
            "Embed should convert to ![[]] syntax, got: {}",
            result
        );
    }

    #[test]
    fn test_http_links_unchanged() {
        let input = "Visit [[https://example.com]] for info.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[https://example.com]]"),
            "HTTP links should be unchanged, got: {}",
            result
        );
    }

    #[test]
    fn test_anchor_links_unchanged() {
        let input = "See [[#section]] below.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[#section]]"),
            "Anchor links should be unchanged, got: {}",
            result
        );
    }

    #[test]
    fn test_task_markers_converted() {
        let input = "- TODO Buy groceries\n- DONE Clean room";
        let result = content::transform(input, &empty_index());
        assert!(result.contains("- [ ] Buy groceries"), "TODO should convert to [ ]");
        assert!(result.contains("- [x] Clean room"), "DONE should convert to [x]");
    }

    #[test]
    fn test_dollar_tokens_escaped() {
        let input = "Token price: $ETH is rising.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("\\$ETH"),
            "Dollar tokens should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_cloze_converted_to_highlight() {
        let input = "The answer is {{cloze 42}}.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("==42=="),
            "Cloze should convert to highlight, got: {}",
            result
        );
    }

    #[test]
    fn test_block_reference_converted() {
        let input = "See ((a1b2c3d4-e5f6-7890-abcd-ef1234567890)).";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[→ block](#^a1b2c3d4-e5f6-7890-abcd-ef1234567890)"),
            "Block ref should convert, got: {}",
            result
        );
    }

    #[test]
    fn test_hiccup_converts_to_html() {
        let input = r#"- [:div [:h2 "brain state 📊"][:ul [:li "pages: 1,299"][:li "words: 33,951"]][:h3 "Text"][:ul [:li "Blocks: 4,809"]]]"#;
        let result = content::transform(input, &empty_index());

        // Should contain h2 as HTML
        assert!(
            result.contains("<h2>brain state 📊</h2>"),
            "Should convert h2 to HTML, got: {}",
            result
        );

        // Should contain h3 as HTML
        assert!(
            result.contains("<h3>Text</h3>"),
            "Should convert h3 to HTML, got: {}",
            result
        );

        // Should contain list items as HTML
        assert!(
            result.contains("<li>pages: 1,299</li>"),
            "Should convert list items to HTML, got: {}",
            result
        );
    }

    #[test]
    fn test_hiccup_simple_list() {
        let input = "[:ul [:li \"item 1\"][:li \"item 2\"]]";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("<li>item 1</li>"),
            "Should convert list to HTML, got: {}",
            result
        );
        assert!(
            result.contains("<li>item 2</li>"),
            "Should convert list to HTML, got: {}",
            result
        );
    }
}

#[cfg(test)]
mod property_tests {
    use crate::page::parse_properties;

    #[test]
    fn test_parse_simple_property() {
        let content = "title:: My Page\n- Content here";
        let (props, remaining) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"My Page".to_string()));
        assert!(remaining.contains("Content here"));
    }

    #[test]
    fn test_parse_multiple_properties() {
        let content = "title:: Test\ntags:: foo, bar\nicon:: 🔵\n\n- Content";
        let (props, _) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"Test".to_string()));
        assert_eq!(props.get("tags"), Some(&"foo, bar".to_string()));
        assert_eq!(props.get("icon"), Some(&"🔵".to_string()));
    }

    #[test]
    fn test_parse_property_with_list_marker() {
        let content = "- title:: My Page\n- Content";
        let (props, _) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"My Page".to_string()));
    }
}

#[cfg(test)]
mod frontmatter_tests {
    use crate::frontmatter;
    use std::collections::HashMap;

    #[test]
    fn test_frontmatter_with_icon() {
        let mut props = HashMap::new();
        props.insert("icon".to_string(), "🔵".to_string());
        props.insert("title".to_string(), "Test Page".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("title: \"🔵 Test Page\""));
        assert!(fm.contains("icon: \"🔵\""));
    }

    #[test]
    fn test_frontmatter_with_tags() {
        let mut props = HashMap::new();
        props.insert("tags".to_string(), "foo, bar, baz".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("tags:"));
        assert!(fm.contains("  - foo"));
        assert!(fm.contains("  - bar"));
        assert!(fm.contains("  - baz"));
    }

    #[test]
    fn test_frontmatter_with_dates() {
        let props = HashMap::new();
        let fm = frontmatter::generate("test", &props, Some(("2025-01-01", "2024-01-01")));
        assert!(fm.contains("modified: 2025-01-01"));
        assert!(fm.contains("created: 2024-01-01"));
    }

    #[test]
    fn test_frontmatter_escapes_quotes() {
        let mut props = HashMap::new();
        props.insert("title".to_string(), "Test \"quoted\" page".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("Test \\\"quoted\\\" page"));
    }
}

#[cfg(test)]
mod favorites_tests {
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_favorites_index_format() {
        // Create temp directories
        let temp = tempdir().unwrap();
        let favorites_dir = temp.path().join("favorites");
        let pages_dir = temp.path().join("pages");
        fs::create_dir_all(&favorites_dir).unwrap();
        fs::create_dir_all(&pages_dir).unwrap();

        // Create a test page
        fs::write(
            pages_dir.join("test-page.md"),
            "---\ntitle: Test\nicon: 🔵\n---\nContent",
        ).unwrap();

        // Create config.edn with favorites
        let config_content = r#"{:favorites ["test-page"]}"#;
        let config_path = temp.path().join("config.edn");
        fs::write(&config_path, config_content).unwrap();

        // Process favorites
        let result = crate::favorites::process_favorites(&config_path, &favorites_dir, &pages_dir);
        assert!(result.is_ok());

        // Check index.md format
        let index_content = fs::read_to_string(favorites_dir.join("index.md")).unwrap();

        // Should have proper wikilink format with ]] not )]
        assert!(
            !index_content.contains(")]"),
            "Index should not contain ')' in wikilinks, got: {}",
            index_content
        );
        assert!(
            index_content.contains("]]"),
            "Index should contain proper ']]' closing, got: {}",
            index_content
        );
    }

    #[test]
    fn test_favorites_with_dots_in_name() {
        let temp = tempdir().unwrap();
        let favorites_dir = temp.path().join("favorites");
        let pages_dir = temp.path().join("pages");
        fs::create_dir_all(&favorites_dir).unwrap();
        fs::create_dir_all(&pages_dir).unwrap();

        // Create a page with dot in name (like cv.land)
        fs::write(
            pages_dir.join("cv.land.md"),
            "---\ntitle: CV Land\n---\nContent",
        ).unwrap();

        let config_content = r#"{:favorites ["cv.land"]}"#;
        let config_path = temp.path().join("config.edn");
        fs::write(&config_path, config_content).unwrap();

        let result = crate::favorites::process_favorites(&config_path, &favorites_dir, &pages_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1, "Should create 1 favorite");

        // Check that favorite file was created with dot preserved in slug
        assert!(favorites_dir.join("cv.land.md").exists(), "Favorite file should preserve dot");
    }

    #[test]
    fn test_favorites_with_spaces_in_name() {
        let temp = tempdir().unwrap();
        let favorites_dir = temp.path().join("favorites");
        let pages_dir = temp.path().join("pages");
        fs::create_dir_all(&favorites_dir).unwrap();
        fs::create_dir_all(&pages_dir).unwrap();

        // Create a page with spaces (pages keep lowercase with spaces)
        fs::write(
            pages_dir.join("github projects.md"),
            "---\ntitle: GitHub Projects\n---\nContent",
        ).unwrap();

        let config_content = r#"{:favorites ["github projects"]}"#;
        let config_path = temp.path().join("config.edn");
        fs::write(&config_path, config_content).unwrap();

        let result = crate::favorites::process_favorites(&config_path, &favorites_dir, &pages_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1, "Should create 1 favorite");

        // Slug converts spaces to dashes
        assert!(favorites_dir.join("github-projects.md").exists(), "Favorite file should use slugified name");
    }

    #[test]
    fn test_get_default_home() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join("config.edn");

        // Config with default-home
        fs::write(&config_path, r#"
{:meta/version 1
 :default-home {:page "cyberia"}}
"#).unwrap();

        let result = crate::favorites::get_default_home(&config_path);
        assert_eq!(result, Some("cyberia".to_string()));
    }

    #[test]
    fn test_get_default_home_skips_comments() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join("config.edn");

        // Config with commented default-home followed by real one
        fs::write(&config_path, r#"
{:meta/version 1
 ;; :default-home {:page "commented"}
 :default-home {:page "actual"}}
"#).unwrap();

        let result = crate::favorites::get_default_home(&config_path);
        assert_eq!(result, Some("actual".to_string()), "Should skip commented lines");
    }

    #[test]
    fn test_get_site_title_from_default_home() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join("config.edn");

        // Config without :meta/title, should fall back to default-home
        fs::write(&config_path, r#"
{:default-home {:page "my site"}}
"#).unwrap();

        let result = crate::favorites::get_site_title(&config_path);
        assert_eq!(result, Some("my site".to_string()));
    }

    #[test]
    fn test_write_site_config() {
        let temp = tempdir().unwrap();
        let config_path = temp.path().join("config.edn");
        let output_dir = temp.path().join("output");
        fs::create_dir_all(&output_dir).unwrap();

        fs::write(&config_path, r#"{:default-home {:page "cyberia"}}"#).unwrap();

        let result = crate::favorites::write_site_config(&config_path, &output_dir);
        assert!(result.is_some());

        let config = result.unwrap();
        assert_eq!(config.page_title, "Cyberia"); // Capitalized
        assert_eq!(config.home_page, "cyberia");

        // Check JSON file was created
        let json_path = output_dir.join("_site_config.json");
        assert!(json_path.exists());

        let json_content = fs::read_to_string(json_path).unwrap();
        assert!(json_content.contains("Cyberia"));
    }
}

#[cfg(test)]
mod journals_tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::config::Config;

    #[test]
    fn test_journals_index_embeds_content() {
        let temp = tempdir().unwrap();
        let journals_dir = temp.path().join("journals");
        let output_dir = temp.path().join("output");
        fs::create_dir_all(&journals_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create a test journal
        fs::write(
            journals_dir.join("2025_01_15.md"),
            "- Did some work today\n- Met with team",
        ).unwrap();

        let config = Config {
            input_dir: temp.path().to_path_buf(),
            output_dir: output_dir.clone(),
            include_private: false,
            create_stubs: false,
            verbose: false,
        };

        let page_index = Vec::new();
        let result = crate::journals::process_journals(&journals_dir, &output_dir, &page_index, &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Check index.md has embed syntax
        let index_content = fs::read_to_string(output_dir.join("index.md")).unwrap();
        assert!(
            index_content.contains("![[journals/2025-01-15]]"),
            "Index should embed journal content, got: {}",
            index_content
        );
        assert!(
            index_content.contains("## [[journals/2025-01-15"),
            "Index should have heading link, got: {}",
            index_content
        );
    }

    #[test]
    fn test_journals_sorted_descending() {
        let temp = tempdir().unwrap();
        let journals_dir = temp.path().join("journals");
        let output_dir = temp.path().join("output");
        fs::create_dir_all(&journals_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create journals in random order
        fs::write(journals_dir.join("2025_01_01.md"), "First").unwrap();
        fs::write(journals_dir.join("2025_01_15.md"), "Middle").unwrap();
        fs::write(journals_dir.join("2025_01_31.md"), "Last").unwrap();

        let config = Config {
            input_dir: temp.path().to_path_buf(),
            output_dir: output_dir.clone(),
            include_private: false,
            create_stubs: false,
            verbose: false,
        };

        let page_index = Vec::new();
        crate::journals::process_journals(&journals_dir, &output_dir, &page_index, &config).unwrap();

        let index_content = fs::read_to_string(output_dir.join("index.md")).unwrap();

        // 2025-01-31 should appear before 2025-01-15 which should appear before 2025-01-01
        let pos_31 = index_content.find("2025-01-31").unwrap();
        let pos_15 = index_content.find("2025-01-15").unwrap();
        let pos_01 = index_content.find("2025-01-01").unwrap();

        assert!(pos_31 < pos_15, "Latest date should come first");
        assert!(pos_15 < pos_01, "Dates should be in descending order");
    }
}

#[cfg(test)]
mod query_tests {
    use crate::page::Page;
    use crate::query;
    use std::collections::HashMap;

    fn create_test_page(name: &str, tags: Vec<&str>) -> Page {
        Page {
            name: name.to_string(),
            name_lower: name.to_lowercase(),
            content: String::new(),
            properties: HashMap::new(),
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
            aliases: vec![],
            namespace: None,
            modified: None,
            created: None,
        }
    }

    #[test]
    fn test_query_page_tags() {
        let pages = vec![
            create_test_page("page1", vec!["rust", "programming"]),
            create_test_page("page2", vec!["rust"]),
            create_test_page("page3", vec!["python"]),
        ];

        let results = query::execute("{{query (page-tags [[rust]])}}", &pages);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_page_tags_strips_pages_prefix() {
        let pages = vec![
            create_test_page("page1", vec!["rust"]),
        ];

        let results = query::execute("{{query (page-tags [[pages/rust]])}}", &pages);
        assert_eq!(results.len(), 1, "Should strip pages/ prefix from query");
    }

    #[test]
    fn test_query_and() {
        let pages = vec![
            create_test_page("page1", vec!["rust", "programming"]),
            create_test_page("page2", vec!["rust"]),
        ];

        let results = query::execute("{{query (and (page-tags [[rust]]) (page-tags [[programming]]))}}", &pages);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "page1");
    }

    #[test]
    fn test_query_results_default_table() {
        let pages = vec![
            create_test_page("my-page", vec!["test"]),
        ];

        let results = query::execute("{{query (page-tags [[test]])}}", &pages);
        let markdown = query::results_to_markdown_with_options(&results, "test query", &query::QueryOptions::default());

        // Default is now table view (like Logseq)
        assert!(
            markdown.contains("| Page |"),
            "Default should be table view, got: {}",
            markdown
        );
        assert!(
            markdown.contains("[[my-page]]"),
            "Table should contain page link, got: {}",
            markdown
        );
    }

    #[test]
    fn test_query_results_explicit_list() {
        let pages = vec![
            create_test_page("my-page", vec!["test"]),
        ];

        let results = query::execute("{{query (page-tags [[test]])}}", &pages);
        let opts = query::QueryOptions {
            table: Some(false),  // Explicitly request list
            ..Default::default()
        };
        let markdown = query::results_to_markdown_with_options(&results, "test query", &opts);

        assert!(
            markdown.contains("- [[my-page|my-page]]"),
            "Should render as list when table=false, got: {}",
            markdown
        );
    }

    #[test]
    fn test_query_nested_and() {
        // Test: (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[research]])))
        let pages = vec![
            create_test_page("page1", vec!["genus", "research"]),           // should match
            create_test_page("page2", vec!["genus", "class", "research"]),  // should NOT (has class)
            create_test_page("page3", vec!["genus"]),                        // should NOT (no research)
            create_test_page("page4", vec!["genus", "research", "other"]),  // should match
        ];

        let results = query::execute(
            "{{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[research]])))}}",
            &pages
        );

        assert_eq!(results.len(), 2, "Should match pages with genus AND research but NOT class");
        let names: Vec<_> = results.iter().map(|p| &p.name).collect();
        assert!(names.contains(&&"page1".to_string()));
        assert!(names.contains(&&"page4".to_string()));
    }

    #[test]
    fn test_query_multiple_nots() {
        // Test: (and (page-tags [[genus]]) (not (page-tags [[class]])) (not (page-tags [[research]])) (not (page-tags [[prohibited]])))
        let pages = vec![
            create_test_page("page1", vec!["genus"]),                              // should match
            create_test_page("page2", vec!["genus", "class"]),                     // should NOT
            create_test_page("page3", vec!["genus", "research"]),                  // should NOT
            create_test_page("page4", vec!["genus", "prohibited"]),                // should NOT
            create_test_page("page5", vec!["genus", "allowed"]),                   // should match
            create_test_page("page6", vec!["genus", "class", "research"]),         // should NOT
        ];

        let results = query::execute(
            "{{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (not (page-tags [[research]])) (not (page-tags [[prohibited]])))}}",
            &pages
        );

        assert_eq!(results.len(), 2, "Should match pages with genus but NOT class, research, or prohibited");
        let names: Vec<_> = results.iter().map(|p| &p.name).collect();
        assert!(names.contains(&&"page1".to_string()));
        assert!(names.contains(&&"page5".to_string()));
    }

    #[test]
    fn test_query_complex_nested_or_and() {
        // Test complex: (or (and (page-tags [[a]]) (page-tags [[b]])) (and (page-tags [[c]]) (page-tags [[d]])))
        let pages = vec![
            create_test_page("page1", vec!["a", "b"]),           // matches first AND
            create_test_page("page2", vec!["c", "d"]),           // matches second AND
            create_test_page("page3", vec!["a"]),                // no match
            create_test_page("page4", vec!["a", "b", "c", "d"]), // matches both
        ];

        let results = query::execute(
            "{{query (or (and (page-tags [[a]]) (page-tags [[b]])) (and (page-tags [[c]]) (page-tags [[d]])))}}",
            &pages
        );

        assert_eq!(results.len(), 3, "Should match pages with (a AND b) OR (c AND d)");
        let names: Vec<_> = results.iter().map(|p| &p.name).collect();
        assert!(names.contains(&&"page1".to_string()));
        assert!(names.contains(&&"page2".to_string()));
        assert!(names.contains(&&"page4".to_string()));
    }

    #[test]
    fn test_query_with_extra_spaces() {
        // Test query with extra spaces before closing parens (common in Logseq)
        let pages = vec![
            create_test_page("page1", vec!["genus", "prohibited"]),
            create_test_page("page2", vec!["genus", "class"]),
            create_test_page("page3", vec!["genus"]),
        ];

        // Query with extra space before closing paren: [[prohibited]] ))
        let results = query::execute(
            "{{query (and (page-tags [[genus]]) (not (page-tags [[class]])) (and (page-tags [[prohibited]] )))}}",
            &pages
        );

        assert_eq!(results.len(), 1, "Should match page with genus+prohibited but not class");
        assert_eq!(results[0].name, "page1");
    }

    #[test]
    fn test_query_with_various_whitespace() {
        // Test query with extra spaces in various positions
        let pages = vec![
            create_test_page("page1", vec!["a", "b"]),
            create_test_page("page2", vec!["a"]),
        ];

        // Extra spaces after keywords
        let results = query::execute(
            "{{query (and   (page-tags [[a]])  (page-tags [[b]]) )}}",
            &pages
        );
        assert_eq!(results.len(), 1, "Should handle extra spaces after 'and'");
        assert_eq!(results[0].name, "page1");

        // Extra spaces in NOT
        let results2 = query::execute(
            "{{query (and (page-tags [[a]]) (not   (page-tags [[b]]) ))}}",
            &pages
        );
        assert_eq!(results2.len(), 1, "Should handle extra spaces after 'not'");
        assert_eq!(results2[0].name, "page2");
    }
}

#[cfg(test)]
mod table_and_pdf_tests {
    use crate::content;
    use crate::page::PageIndex;

    fn empty_index() -> PageIndex {
        Vec::new()
    }

    // ===========================================
    // Table Tests
    // ===========================================

    #[test]
    fn test_table_with_malformed_separator_fewer_columns() {
        // Logseq sometimes has separator rows with fewer columns than the header
        // The fix should detect this and generate a correct separator
        let input = r#"- | Col1 | Col2 | Col3 | Col4 | Col5 |
  | ---- | ---- |
  | val1 | val2 | val3 | val4 | val5 |"#;

        let result = content::transform(input, &empty_index());

        // Should have a 5-column separator, not the malformed 2-column one
        assert!(
            result.contains("|---|---|---|---|---|"),
            "Should generate correct 5-column separator, got: {}",
            result
        );
        // The malformed separator should be removed
        assert!(
            !result.contains("| ---- | ---- |"),
            "Should remove malformed 2-column separator, got: {}",
            result
        );
    }

    #[test]
    fn test_table_with_correct_separator_unchanged() {
        // Tables with correct separators should be left unchanged
        let input = r#"- | Col1 | Col2 | Col3 |
  |------|------|------|
  | val1 | val2 | val3 |"#;

        let result = content::transform(input, &empty_index());

        // Should preserve the existing correct separator
        assert!(
            result.contains("|------|------|------|"),
            "Should preserve correct separator, got: {}",
            result
        );
    }

    #[test]
    fn test_table_9_columns_with_3_column_separator() {
        // Real-world test case: 9-column table with 3-column separator (from Logseq)
        let input = r#"- | Aspect | No | Parameters | Col4 | Col5 | Col6 | Col7 | Col8 | Col9 |
  | ---- | ---- | ---- |
  | Heavy Metals | 1 | Lead (Pb) | 29.318 | 29.328 | 29.032 | 28.365 | 31.165 | 30.454 |"#;

        let result = content::transform(input, &empty_index());

        // Should have a 9-column separator
        assert!(
            result.contains("|---|---|---|---|---|---|---|---|---|"),
            "Should generate correct 9-column separator, got: {}",
            result
        );
        // Data row should be preserved
        assert!(
            result.contains("| Heavy Metals | 1 | Lead (Pb) |"),
            "Should preserve data rows, got: {}",
            result
        );
    }

    // ===========================================
    // PDF Image Syntax Tests
    // ===========================================

    #[test]
    fn test_pdf_image_syntax_converted_to_iframe() {
        // Logseq uses image syntax for PDFs: ![name.pdf](path.pdf)
        let input = "- ![document.pdf](../assets/document.pdf)";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains(r#"<iframe src="../assets/document.pdf" width="100%" height="600px"#),
            "PDF image syntax should convert to iframe, got: {}",
            result
        );
        // Should not contain the original image syntax
        assert!(
            !result.contains("![document.pdf]"),
            "Should not contain original image syntax, got: {}",
            result
        );
    }

    #[test]
    fn test_pdf_image_syntax_with_empty_alt() {
        // PDF with empty alt text: ![](path.pdf)
        let input = "- ![](../assets/report.pdf)";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains(r#"<iframe src="../assets/report.pdf" width="100%" height="600px"#),
            "PDF with empty alt should convert to iframe, got: {}",
            result
        );
    }

    #[test]
    fn test_pdf_logseq_syntax_still_works() {
        // Original {{pdf ...}} syntax should still work
        let input = "- {{pdf ../assets/document.pdf}}";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains(r#"<iframe src="../assets/document.pdf" width="100%" height="600px"#),
            "{{pdf}} syntax should convert to iframe, got: {}",
            result
        );
    }

    #[test]
    fn test_regular_image_not_converted_to_iframe() {
        // Regular images should not be converted to iframes
        let input = "- ![photo.png](../assets/photo.png)";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("![photo.png](../assets/photo.png)"),
            "Regular images should remain unchanged, got: {}",
            result
        );
        assert!(
            !result.contains("<iframe"),
            "Regular images should not become iframes, got: {}",
            result
        );
    }

    // ===========================================
    // Wikilink Prefix Matching Tests
    // ===========================================

    fn create_page(name: &str) -> crate::page::Page {
        create_page_with_aliases(name, vec![])
    }

    fn create_page_with_aliases(name: &str, aliases: Vec<&str>) -> crate::page::Page {
        crate::page::Page {
            name: name.to_string(),
            name_lower: name.to_lowercase(),
            tags: vec![],
            properties: std::collections::HashMap::new(),
            content: String::new(),
            aliases: aliases.into_iter().map(|s| s.to_string()).collect(),
            namespace: None,
            modified: None,
            created: None,
        }
    }

    #[test]
    fn test_wikilink_prefix_match_visit_us_to_visit() {
        // "visit us" should match "visit" page when "visit us" doesn't exist
        let page_index = vec![create_page("visit"), create_page("other page")];
        let input = "- Check out [[visit us]] for info";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[visit|visit us]]"),
            "Should rewrite [[visit us]] to [[visit|visit us]], got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_exact_match_not_rewritten() {
        // Exact match should not be rewritten
        let page_index = vec![create_page("visit"), create_page("visit us")];
        let input = "- Check out [[visit us]] for info";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[visit us]]"),
            "Exact match should not be rewritten, got: {}",
            result
        );
        assert!(
            !result.contains("[[visit|visit us]]"),
            "Should not add alias for exact match, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_prefix_match_preserves_existing_alias() {
        // If link already has an alias, preserve it
        let page_index = vec![create_page("visit")];
        let input = "- Check out [[visit us|come see us]] for info";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[visit|come see us]]"),
            "Should preserve existing alias when rewriting link, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_prefix_match_longest_wins() {
        // "cyber valley estate" should match "cyber valley" not "cyber"
        let page_index = vec![
            create_page("cyber"),
            create_page("cyber valley"),
            create_page("other"),
        ];
        let input = "- Visit [[cyber valley estate]] today";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[cyber valley|cyber valley estate]]"),
            "Should match longest prefix 'cyber valley', got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_no_match_unchanged() {
        // No matching page - link should remain unchanged
        let page_index = vec![create_page("other"), create_page("something")];
        let input = "- Check out [[completely different]] for info";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[completely different]]"),
            "Non-matching link should remain unchanged, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_prefix_match_case_insensitive() {
        // Matching should be case-insensitive
        let page_index = vec![create_page("Visit")];
        let input = "- Check out [[visit us]] for info";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[Visit|visit us]]"),
            "Prefix matching should be case-insensitive, got: {}",
            result
        );
    }

    #[test]
    fn test_markdown_link_with_wikilink_url() {
        // Logseq syntax [text]([[Page]]) should convert to [text](Page)
        let input = "- Check out [our tasks]([[Tasks]]) for examples";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("[our tasks](Tasks)"),
            "Markdown link with wikilink URL should be converted, got: {}",
            result
        );
        assert!(
            !result.contains("[[Tasks]]"),
            "Should not contain wikilink syntax in URL, got: {}",
            result
        );
    }

    // ===========================================
    // Alias Resolution Tests
    // ===========================================

    #[test]
    fn test_alias_exact_match() {
        // Link "cv/districts" should match page with alias "cv/districts"
        let page_index = vec![
            create_page_with_aliases("cyber valley/districts", vec!["cv/districts"]),
            create_page("other page"),
        ];
        let input = "- Discover [[cv/districts]] here";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[cyber valley/districts|cv/districts]]"),
            "Should resolve alias to page name, got: {}",
            result
        );
    }

    #[test]
    fn test_alias_simple_match() {
        // Link "cv" should match page "cyber valley" with alias "cv"
        let page_index = vec![
            create_page_with_aliases("cyber valley", vec!["cv", "about"]),
            create_page("other"),
        ];
        let input = "- Visit [[cv]] today";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[cyber valley|cv]]"),
            "Should resolve alias 'cv' to 'cyber valley', got: {}",
            result
        );
    }

    #[test]
    fn test_namespace_alias_expansion() {
        // Link "cv/districts" where "cv" is alias for "cyber valley"
        // should match "cyber valley/districts"
        let page_index = vec![
            create_page_with_aliases("cyber valley", vec!["cv"]),
            create_page("cyber valley/districts"),
        ];
        let input = "- Discover [[cv/districts]] here";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[cyber valley/districts|cv/districts]]"),
            "Should expand namespace alias, got: {}",
            result
        );
    }

    #[test]
    fn test_alias_does_not_override_exact_page() {
        // If both page "cv" and alias "cv" exist, page should win
        let page_index = vec![
            create_page("cv"),
            create_page_with_aliases("cyber valley", vec!["cv"]),
        ];
        let input = "- Visit [[cv]] today";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[cv]]"),
            "Exact page match should take priority over alias, got: {}",
            result
        );
        assert!(
            !result.contains("[[cyber valley|cv]]"),
            "Should not rewrite when exact page exists, got: {}",
            result
        );
    }

    #[test]
    fn test_multiple_aliases() {
        // Page with multiple aliases
        let page_index = vec![
            create_page_with_aliases("visit", vec!["residency", "come visit"]),
        ];

        let input1 = "- Check [[residency]] options";
        let result1 = content::transform(input1, &page_index);
        assert!(
            result1.contains("[[visit|residency]]"),
            "Should resolve first alias, got: {}",
            result1
        );

        let input2 = "- Please [[come visit]] us";
        let result2 = content::transform(input2, &page_index);
        assert!(
            result2.contains("[[visit|come visit]]"),
            "Should resolve second alias, got: {}",
            result2
        );
    }

    #[test]
    fn test_alias_case_insensitive() {
        // Alias matching should be case-insensitive
        let page_index = vec![
            create_page_with_aliases("Cyber Valley", vec!["CV"]),
        ];
        let input = "- Visit [[cv]] today";
        let result = content::transform(input, &page_index);

        assert!(
            result.contains("[[Cyber Valley|cv]]"),
            "Alias matching should be case-insensitive, got: {}",
            result
        );
    }

    #[test]
    fn test_dollar_currency_escaped() {
        // Currency amounts should be escaped to prevent LaTeX interpretation
        let input = "- The price is $100 USD";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("\\$100"),
            "Currency $100 should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_dollar_currency_with_comma_escaped() {
        // Currency with thousands separator should be escaped
        let input = "- Budget: $50,000";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("\\$50,000"),
            "Currency $50,000 should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_dollar_currency_with_decimal_escaped() {
        // Currency with decimal should be escaped
        let input = "- Price: $19.99";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("\\$19.99"),
            "Currency $19.99 should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_dollar_currency_with_suffix_escaped() {
        // Currency with k/M/B suffix should be escaped
        let input = "- Cost: $10k to $7M";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("\\$10k") && result.contains("\\$7M"),
            "Currency with suffix should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_math_mode_not_escaped() {
        // LaTeX math mode $...$ should NOT be escaped
        let input = "- Inline math: $x^2 + y^2 = z^2$";
        let result = content::transform(input, &empty_index());

        // The $ before x should not be escaped (it's math mode, not currency)
        // Note: The current implementation may escape this - if so, we need smarter detection
        assert!(
            result.contains("$x^2"),
            "Math mode should be preserved, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_dollar_not_escaped() {
        // Dollar signs inside wikilinks should NOT be escaped
        // Page names like $BOOT.md need the wikilink to match exactly
        let input = "- [[$BOOT]] is the token and [[$V]] is will";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("[[$BOOT]]") && result.contains("[[$V]]"),
            "Dollar signs in wikilinks should NOT be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_dollar_token_outside_wikilink_escaped() {
        // Dollar signs OUTSIDE wikilinks should be escaped
        let input = "- Use $BOOT for staking, see [[$BOOT]] for details";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("\\$BOOT for staking") && result.contains("[[$BOOT]]"),
            "Dollar in text escaped, in wikilink not, got: {}",
            result
        );
    }

    #[test]
    fn test_embed_wikilink_dollar_not_escaped() {
        // Embed syntax ![[...]] should also preserve dollar signs
        let input = "- ![[Finalization of $BOOT distribution]]";
        let result = content::transform(input, &empty_index());

        assert!(
            result.contains("$BOOT"),
            "Dollar signs in embed wikilinks should NOT be escaped, got: {}",
            result
        );
    }
}
