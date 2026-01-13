# Changelog

## [0.3.0] - 2025-01-13

### Added
- Query results now render as tables by default (like Logseq)
- Auto-detect properties for table columns (Page, Tags, and common properties)
- Complex nested query support: AND with NOT, multiple NOTs, nested ANDs/ORs
- `query-table:: false` option to force list view instead of table
- Stub pages for `$` prefixed links (e.g., `[[$CYBER]]` creates `$cyber.md`)
- Example pages demonstrating complex queries and manual tables

### Fixed
- Tables now render properly with blank line before for markdown parsing
- Stub page naming for escaped dollar signs (`\$` → `$`)

### Changed
- Default query output changed from list to table view

## [0.2.1] - 2025-01-13

### Changed
- Favorites now redirect to actual pages instead of showing embedded copies
- Clicking a favorite in the Explorer opens the original page directly (like Logseq)
- Removed intermediate embed files from favorites folder

### Added
- Redirect component for handling page redirects via frontmatter

## [0.2.0] - 2025-01-13

### Added
- Query support with build-time execution
- Example graph for testing all features
- Improved setup and documentation

## [0.1.4] - Previous releases

See git history for earlier changes.
