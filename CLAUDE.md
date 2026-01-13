# Publish-Quartz Project Context

## Overview
A fast Logseq-to-Quartz converter that transforms Logseq graphs into static websites. Processes ~3000 pages in under 1 second using a Rust preprocessor.

## Architecture

```
publish-quartz/
├── preprocessor/         # Rust CLI tool (core conversion engine)
│   └── src/
│       ├── main.rs       # CLI entry point, 8-step pipeline
│       ├── content.rs    # Logseq → Quartz markdown transforms (~30 regex patterns)
│       ├── query.rs      # Executes Logseq queries at build time
│       ├── page.rs       # Page parsing, indexing, git metadata
│       ├── journals.rs   # Journal processing
│       ├── favorites.rs  # Extracts favorites from config.edn
│       ├── frontmatter.rs# YAML frontmatter generation
│       └── tests.rs      # Test suite
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
├── action.yml            # GitHub Action for CI/CD
├── Makefile              # Local development automation
└── .nvmrc                # Node.js 24 required
```

## Key Commands

```bash
# Build example site
make build

# Start dev server (http://localhost:8080)
make serve

# Quick reprocess content only
make reprocess

# Watch for changes (requires: brew install entr)
make watch-content        # Watch example/ folder
make watch-preprocessor   # Watch Rust source

# Run tests
make test

# Clean everything
make clean
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

| Feature | Implementation |
|---------|----------------|
| Wikilinks `[[page]]` | content.rs - WIKILINK_RE |
| Embeds `{{embed [[page]]}}` | content.rs - EMBED_RE |
| Properties `key:: value` | content.rs - USER_PROPS_RE |
| Tasks (TODO/DONE/LATER/etc) | content.rs - task markers |
| Priority `[#A]` `[#B]` `[#C]` | content.rs - PRIORITY_*_RE |
| Queries `{{query ...}}` | query.rs - execute() |
| Hiccup `[:h2 "text"]` | content.rs - convert_hiccup_to_markdown() |
| Cloze `{{cloze text}}` | content.rs - CLOZE_RE |
| YouTube/Video/PDF embeds | content.rs - media patterns |
| Scheduled/Deadline | content.rs - SCHEDULED_RE/DEADLINE_RE |
| Dollar sign escaping | content.rs - DOLLAR_TOKEN_RE |

## Development Notes

- **Always use `make` commands** - Use `make build`, `make serve`, `make reprocess`, etc. for all operations
- **Node.js 24 required** - Run `source ~/.nvm/nvm.sh && nvm use 24` before make commands if Node version errors occur
- **Quartz cloned to `quartz-build/`** - not committed, generated at build
- **Content goes to `quartz-build/content/`** - Quartz watches this for hot reload
- **Theme files copied from `quartz-theme/`** - must run `make copy-theme` after changes

## Recent Fixes
- Double bullet fix in query results (content.rs:176-182)
- Dollar sign escaping in wikilinks and YAML titles
- Explorer link fix for folder pages (path.ts)
- Hiccup conversion improvements
- Pages at content root (no pages/ subfolder)

## Testing the Example

1. `make serve` - starts dev server
2. Open http://localhost:8080
3. Navigate to "Queries Demo" to test query rendering
4. Check "Tasks" page for task marker conversion
5. Check "Hiccup Examples" for EDN syntax conversion
