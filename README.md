# publish-quartz

Convert your Logseq graph to a fast, static Quartz website.

Unlike [logseq/publish-spa](https://github.com/logseq/publish-spa) which loads the entire graph JSON on every page visit, this generates individual HTML pages - much faster for large graphs.

## Features

- Fast Rust preprocessor (~3000 pages in <1 second)
- Converts Logseq markdown to Quartz-compatible format
- Handles wikilinks, embeds, queries, properties
- Generates backlinks, graph view, search
- Custom theme with Logseq-style navigation

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
          repository: user/publish-quartz
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

Requirements: Rust, Node.js 22+

```bash
# Clone this repo
git clone https://github.com/user/publish-quartz.git
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

# Build site
cd quartz-build && npx quartz build

# Preview
npx quartz build --serve
```

### Option 3: Use with existing Logseq repo (like cyber)

Add to your Logseq graph's `logseq/config.edn`:

```clojure
:hidden ["/quartz-build"]
```

Create a `Makefile`:

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
| `[[page]]` | `[[pages/page]]` (wikilink) |
| `{{embed [[page]]}}` | `![[pages/page]]` (transclusion) |
| `key:: value` | YAML frontmatter |
| `{{query ...}}` | Executed and rendered as list |
| `NOW/LATER/TODO/DONE` | Checkbox markers |
| `((block-ref))` | Blockquote with link |
| `[:div ...]` (Hiccup) | Code block |

## Configuration

Edit `quartz-theme/quartz.config.ts` to customize:
- Site title, base URL
- Theme colors
- Enabled plugins

## License

MIT
