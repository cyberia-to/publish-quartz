# Changelog

## [0.3.10] - 2025-01-18

### Fixed
- Dollar sign wikilinks now render correctly without KaTeX math interpretation
- `[[$C|$TOCYB]]` no longer shows red math error - displays as clickable `$TOCYB` link
- Wikilinks with `$` output as raw HTML anchors to bypass KaTeX processing order issue

### Changed
- Dollar wikilinks (`[[$V]]`, `[[$C|$TOCYB]]`) now output `<a>` tags instead of `[[...]]` syntax
- Stub extraction updated to handle both wikilink syntax and HTML anchors

## [0.3.9] - 2025-01-18

### Fixed
- Dollar signs in wikilink aliases now escaped for LaTeX compatibility
- `[[$TOCYB]]` resolving to `$C` now displays correctly as `[[$C|\$TOCYB]]`
- Simple wikilinks like `[[$V]]` remain unescaped for proper page matching

### Added
- CI now runs tests before building release binaries
- Pre-commit hook runs `cargo test` before commits
- `make setup-hooks` command to install git hooks after cloning

## [0.3.8] - 2025-01-18

### Fixed
- Wikilinks to pages with `$` in name now work correctly (e.g., `[[$V]]` → `$V.md`)
- Dollar signs in text are escaped for LaTeX (`$100` → `\$100`)
- Wikilinks are protected from dollar escaping using placeholder strategy

## [0.3.7] - 2025-01-18

### Fixed
- Currency amounts now escaped for LaTeX compatibility (`$100`, `$50,000`)
- Currency with suffixes also escaped (`$10k`, `$7M`, `$100k`)

## [0.3.6] - 2025-01-18

### Added
- Alias resolution for wikilinks (e.g., `[[cv/districts]]` resolves via page aliases)
- Namespace alias expansion (`cv/X` expands to `cyber valley/X` when `cv` is alias)
- Four-level matching: exact page → exact alias → namespace expansion → prefix match

## [0.3.5] - 2025-01-18

### Added
- Wikilink prefix matching (`[[visit us]]` matches `visit` page if exact match not found)
- Markdown link with wikilink URL support: `[text]([[Page]])` → `[text](Page)`

## [0.3.4] - 2025-01-18

### Fixed
- Table separator validation now checks column count matches header
- PDF files with image syntax `![name.pdf](path.pdf)` now render as iframes
- CSS and assets now load correctly on GitHub Pages subdirectory deployments
- Added `<base>` tag for subdirectory deployments to fix relative path resolution
- Fixes 404 errors for `index.css`, `prescript.js`, etc. when deployed to `username.github.io/repo/`

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
