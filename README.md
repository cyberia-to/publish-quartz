# publish-quartz

Convert your Logseq graph to a fast, static Quartz website.

**[Live Example](https://cyberia-to.github.io/publish-quartz/)** - See it in action

Unlike [logseq/publish-spa](https://github.com/logseq/publish-spa) which loads the entire graph JSON on every page visit, this generates individual HTML pages - much faster for large graphs.

## Features

- Fast Rust preprocessor (~3000 pages in <1 second)
- Converts Logseq markdown to Quartz-compatible format
- Handles wikilinks, embeds, queries, properties, tasks
- Generates backlinks, graph view, search
- Custom theme with Logseq-style navigation

## Quick Start

Try it locally with the included example graph:

```bash
git clone https://github.com/cyberia-to/publish-quartz.git
cd publish-quartz
nvm use        # Uses .nvmrc (Node 24)
make serve     # Build and start dev server
```

Open http://localhost:8080 to see the example site.

**Requirements:** [Rust](https://rustup.rs/) and [Node.js 22+](https://nodejs.org/) (24 recommended)

## Development

```bash
# Install git hooks (runs tests before each commit)
make setup-hooks

# Run tests
make test

# Build preprocessor
make build-preprocessor
```

## Usage

### Option 1: GitHub Action

Add `.github/workflows/publish.yml` to your Logseq graph repo:

```yaml
name: Publish to GitHub Pages

on:
  push:
    branches: [main, master]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: actions/checkout@v4
        with:
          repository: cyberia-to/publish-quartz
          path: publish-quartz

      - uses: ./publish-quartz
        with:
          graph-path: '.'
          output-path: 'public'

      - uses: actions/upload-pages-artifact@v3
        with:
          path: public

  deploy:
    runs-on: ubuntu-latest
    needs: build
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - uses: actions/deploy-pages@v4
```

### Option 2: Local CLI

Process your own Logseq graph:

```bash
# Clone this repo
git clone https://github.com/cyberia-to/publish-quartz.git
cd publish-quartz

# Build preprocessor
cd preprocessor && cargo build --release && cd ..

# Clone Quartz
git clone https://github.com/jackyzha0/quartz.git quartz-build
cd quartz-build && npm ci && cd ..

# Process your graph
./preprocessor/target/release/logseq-to-quartz \
  --input /path/to/your/logseq-graph \
  --output quartz-build/content \
  --create-stubs

# Copy theme
cp quartz-theme/quartz.config.ts quartz-build/
cp quartz-theme/quartz.layout.ts quartz-build/
cp quartz-theme/styles/*.scss quartz-build/quartz/styles/
cp quartz-theme/components/*.tsx quartz-build/quartz/components/
cp quartz-theme/scripts/*.ts quartz-build/quartz/components/scripts/

# Build and preview
cd quartz-build && npx quartz build --serve
```

### Option 3: Integrate with your Logseq repo

Add to your Logseq graph's `logseq/config.edn`:

```clojure
:hidden ["/quartz-build"]
```

Create a `Makefile` in your graph repo:

```makefile
LIBRARY := ../publish-quartz
PREPROCESSOR := $(LIBRARY)/preprocessor/target/release/logseq-to-quartz
THEME := $(LIBRARY)/quartz-theme

setup:
	@if [ ! -d "quartz-build" ]; then \
		git clone https://github.com/jackyzha0/quartz.git quartz-build; \
		cd quartz-build && npm ci; \
	fi
	cd $(LIBRARY)/preprocessor && cargo build --release

build:
	rm -rf quartz-build/content
	$(PREPROCESSOR) --input . --output quartz-build/content --create-stubs
	cp $(THEME)/quartz.config.ts quartz-build/
	cp $(THEME)/quartz.layout.ts quartz-build/
	cd quartz-build && npx quartz build

serve:
	cd quartz-build && npx quartz build --serve
```

Then run:
```bash
make setup  # One-time setup
make serve  # Build and preview
```

## What gets converted

| Logseq | Quartz |
|--------|--------|
| `[[page]]` | Wikilink with alias resolution |
| `[[$TOKEN]]` | Links to pages with $ in name |
| `{{embed [[page]]}}` | Transclusion |
| `key:: value` | YAML frontmatter / inline display |
| `alias:: name` | Page aliases for wikilink resolution |
| `{{query ...}}` | Executed at build time, rendered as list/table |
| `{{youtube URL}}` | Embedded video |
| `{{pdf URL}}` | Embedded PDF iframe |
| `![doc.pdf](path.pdf)` | Embedded PDF iframe |
| `TODO/DOING/DONE/LATER` | Checkbox markers with icons |
| `[#A]` `[#B]` `[#C]` | Priority indicators |
| `SCHEDULED:` `DEADLINE:` | Date badges |
| `((block-ref))` | Blockquote with link |
| `[:div ...]` (Hiccup) | Converted to HTML |
| Tables in bullets | Proper markdown tables |
| `$100`, `$TOKEN` | Escaped for LaTeX compatibility |

## Configuration

Edit `quartz-theme/quartz.config.ts` to customize:
- Site title, base URL
- Theme colors
- Enabled plugins

Logseq config options read from `logseq/config.edn`:
- `:favorites` - Pinned pages in sidebar
- `:default-home` - Home page

## License

MIT
