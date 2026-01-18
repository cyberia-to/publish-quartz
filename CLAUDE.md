# Publish-Quartz Project Context

## Overview
A fast Logseq-to-Quartz converter that transforms Logseq graphs into static websites. Processes ~3000 pages in under 1 second using a Rust preprocessor.

**Live Demo:** https://cyberia-to.github.io/publish-quartz/
**Repository:** https://github.com/cyberia-to/publish-quartz

## Architecture

```
publish-quartz/
├── preprocessor/         # Rust CLI tool (core conversion engine)
│   └── src/
│       ├── main.rs       # CLI entry point, 8-step pipeline
│       ├── content.rs    # Logseq → Quartz markdown transforms (~30 regex patterns)
│       ├── query.rs      # Executes Logseq queries at build time
│       ├── page.rs       # Page parsing, indexing, git metadata, aliases
│       ├── journals.rs   # Journal processing
│       ├── favorites.rs  # Extracts favorites from config.edn
│       ├── frontmatter.rs# YAML frontmatter generation
│       ├── config.rs     # Configuration handling
│       └── tests.rs      # Test suite (70+ tests)
├── quartz-theme/         # Custom Quartz theme (TypeScript/SCSS)
│   ├── components/       # React components (Favorites, Journals, etc.)
│   ├── scripts/          # Inline scripts
│   ├── styles/           # SCSS styles
│   ├── quartz.config.ts  # Site configuration
│   └── quartz.layout.ts  # Page layout
├── example/              # Example Logseq graph for testing
│   ├── pages/            # Sample pages with all features
│   ├── journals/         # Sample journal entries
│   ├── logseq/config.edn # Logseq configuration
│   └── assets/           # Static assets
├── scripts/              # Development scripts
│   ├── pre-commit        # Git pre-commit hook (runs tests)
│   └── setup-hooks.sh    # Install git hooks
├── .github/workflows/
│   └── release.yml       # CI: builds binaries, runs tests, deploys demo
├── action.yml            # GitHub Action for CI/CD
├── Makefile              # Local development automation
└── .nvmrc                # Node.js 24 required
```

## Key Commands

```bash
# Development setup (run once after cloning)
make setup-hooks          # Install pre-commit hooks

# Build and serve
make build                # Full build: preprocess + build Quartz
make serve                # Start dev server (http://localhost:8080)
make reprocess            # Quick reprocess content only

# Testing
make test                 # Run preprocessor tests (cargo test)

# Watch for changes (requires: brew install entr)
make watch-content        # Watch example/ folder
make watch-preprocessor   # Watch Rust source

# Cleanup
make clean                # Remove all generated files
make clean-content        # Remove content only (keep deps)
```

## Preprocessor CLI

```bash
./preprocessor/target/release/logseq-to-quartz \
  --input <logseq-graph-path> \
  --output <quartz-content-path> \
  --create-stubs \
  --verbose
```

## Supported Logseq Features

| Feature | Implementation | Notes |
|---------|----------------|-------|
| Wikilinks `[[page]]` | content.rs - WIKILINK_RE | With alias resolution |
| Pages with `$` names | `[[$V]]` → `$V.md` | Dollar signs preserved in links |
| Aliases `alias:: name` | page.rs - extract_aliases() | Resolves `[[alias]]` to actual page |
| Prefix matching | content.rs - find_best_page_match() | `[[visit us]]` matches `visit` |
| Namespace expansion | `[[cv/X]]` → `[[cyber valley/X]]` | When `cv` is alias |
| Embeds `{{embed [[page]]}}` | content.rs - EMBED_RE | Transclusion |
| Properties `key:: value` | content.rs - USER_PROPS_RE | Frontmatter + inline |
| Tasks (TODO/DONE/LATER/etc) | content.rs - task markers | Checkbox with icons |
| Priority `[#A]` `[#B]` `[#C]` | content.rs - PRIORITY_*_RE | Color indicators |
| Queries `{{query ...}}` | query.rs - execute() | Build-time execution |
| Tables in bullets | content.rs - fix_tables() | Validates separator columns |
| Hiccup `[:h2 "text"]` | content.rs - convert_hiccup_to_markdown() | EDN → HTML |
| Cloze `{{cloze text}}` | content.rs - CLOZE_RE | Highlight syntax |
| YouTube/Video embeds | content.rs - YOUTUBE_RE/VIDEO_RE | |
| PDF embeds | content.rs - PDF_RE, IMAGE_PDF_RE | `{{pdf}}` and `![.pdf]()` |
| Scheduled/Deadline | content.rs - SCHEDULED_RE/DEADLINE_RE | Date badges |
| Dollar sign escaping | content.rs - escape_dollars_outside_wikilinks() | `$100` → `\$100` |

## Dollar Sign Handling

Critical for LaTeX compatibility:

| Context | Example | Behavior |
|---------|---------|----------|
| Simple wikilink | `[[$V]]` | NOT escaped (page matching) |
| Alias wikilink | `[[$C\|$TOCYB]]` | Display text escaped |
| Currency in text | `$100`, `$10k` | Escaped `\$100` |
| Token in text | `$BOOT` | Escaped `\$BOOT` |

Implementation: `escape_dollars_outside_wikilinks()` uses placeholder strategy to protect wikilinks before escaping.

## GitHub Actions & CI

### Release Workflow (`.github/workflows/release.yml`)

Triggered by: pushing a tag `v*` (e.g., `v0.3.9`)

Jobs:
1. **build-binaries**: Builds for linux-x86_64, macos-x86_64, macos-aarch64
   - Runs `cargo test` before building
2. **create-release**: Creates GitHub release with binaries
3. **build-site**: Builds demo site from `example/`
4. **deploy-site**: Deploys to GitHub Pages

### Reusable Workflow

Located in: `cyberia-to/.github` repo
File: `.github/workflows/publish-to-netlify.yml`

Used by consumer repos (cyber, cvland) to build and deploy.

## Release Process

```bash
# 1. Make changes to preprocessor/
# 2. Run tests
make test

# 3. Commit (pre-commit hook runs tests)
git add .
git commit -m "Description"

# 4. Push to main
git push origin main

# 5. Create and push tag
git tag v0.3.X
git push origin v0.3.X

# 6. Update reusable workflow (in cyberia-to/.github)
# Edit .github/workflows/publish-to-netlify.yml
# Change: uses: cyberia-to/publish-quartz@v0.3.X

# 7. Trigger consumer repos rebuild
cd ../cyber && git commit --allow-empty -m "Rebuild" && git push
cd ../cvland && git commit --allow-empty -m "Rebuild" && git push
```

## Consumer Repos

| Repo | Website | Workflow |
|------|---------|----------|
| cyberia-to/cyber | https://cyber.page | Uses reusable workflow |
| cyberia-to/cvland | https://cv.land | Uses reusable workflow |

Both use `cyberia-to/.github/.github/workflows/publish-to-netlify.yml`

## Development Notes

- **Always use `make` commands** - Use `make build`, `make serve`, `make reprocess`, etc.
- **Node.js 24 required** - Run `nvm use` (uses .nvmrc)
- **Pre-commit hook** - Runs tests automatically before commits
- **Quartz cloned to `quartz-build/`** - not committed, generated at build
- **Content goes to `quartz-build/content/`** - Quartz watches this for hot reload
- **Theme files copied from `quartz-theme/`** - must run `make copy-theme` after changes

## Testing

```bash
# Run all tests
make test
# or
cd preprocessor && cargo test

# Run specific test
cd preprocessor && cargo test test_name

# Run tests with output
cd preprocessor && cargo test -- --nocapture
```

Test file: `preprocessor/src/tests.rs` (70+ tests covering all features)

## Troubleshooting

### Node version errors
```bash
nvm use 24
```

### Quartz not found
```bash
make setup-quartz
```

### Tests failing on commit
```bash
make test  # Run manually to see errors
```

### GitHub Pages deployment failing
Check environment protection rules at:
`Settings → Environments → github-pages → Deployment branches and tags`
Must allow `v*` tags.

## Recent Changes (v0.3.4 - v0.3.9)

- **v0.3.9**: CI tests, pre-commit hooks, wikilink alias $ escaping
- **v0.3.8**: Wikilinks to $ pages fixed, placeholder strategy
- **v0.3.7**: Currency escaping ($100, $10k, $7M)
- **v0.3.6**: Alias resolution, namespace expansion
- **v0.3.5**: Prefix matching, `[text]([[Page]])` syntax
- **v0.3.4**: Table column validation, PDF image syntax
