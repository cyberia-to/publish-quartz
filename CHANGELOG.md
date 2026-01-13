# Changelog

## [0.3.3] - 2025-01-13

### Fixed
- Explorer navigation now works correctly for pages with both content and children
- Clicking on unrelated pages from folder pages no longer prepends the current path
- Example: On `/bostrom/` clicking `cv/districts` now correctly goes to `/cv/districts` instead of `/bostrom/cv/districts`
- Fixed by passing `allSlugs` to `resolveRelative` in explorer for proper folder detection

## [0.3.2] - 2025-01-13

### Fixed
- Query parsing now handles extra whitespace anywhere in expressions
- Works with: `(and   ...)`, `(not   ...)`, `(page-tags [[tag]] )`, etc.
- Flexible keyword matching no longer uses hardcoded string offsets

## [0.3.1] - 2025-01-13

### Fixed
- Namespace stub pages now create proper folder structure (`[[cyber/tokens]]` → `cyber/tokens.md`)

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
