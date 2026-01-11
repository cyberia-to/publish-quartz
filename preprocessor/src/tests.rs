#[cfg(test)]
mod path_tests {
    use crate::content;
    use crate::page::PageIndex;

    fn empty_index() -> PageIndex {
        Vec::new()
    }

    #[test]
    fn test_wikilink_adds_pages_prefix() {
        let input = "Check out [[devops]] for more info.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[pages/devops]]"),
            "Expected pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_namespace_gets_prefix() {
        let input = "See [[terrabyte/garden]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[pages/terrabyte/garden]]"),
            "Namespace pages should get pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_preserves_pages_prefix() {
        let input = "See [[pages/cyber]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[pages/cyber]]"),
            "Should preserve existing pages/ prefix, got: {}",
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
            result.contains("[[pages/devops|DevOps Guide]]"),
            "Should preserve alias with pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_embed_adds_pages_prefix() {
        let input = "{{embed [[intro]]}}";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("![[pages/intro]]"),
            "Embed should have pages/ prefix, got: {}",
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
    fn test_query_results_include_pages_prefix() {
        let pages = vec![
            create_test_page("my-page", vec!["test"]),
        ];

        let results = query::execute("{{query (page-tags [[test]])}}", &pages);
        let markdown = query::results_to_markdown(&results, "test query");

        assert!(
            markdown.contains("[[pages/my-page"),
            "Results should include pages/ prefix, got: {}",
            markdown
        );
    }
}
