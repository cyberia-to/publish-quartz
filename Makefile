# Makefile for publish-quartz local development

# Check Node version (requires 22+)
NODE_VERSION := $(shell node -v 2>/dev/null | sed 's/v//' | cut -d. -f1)
NODE_OK := $(shell [ "$(NODE_VERSION)" -ge 22 ] 2>/dev/null && echo yes || echo no)

check-node:
	@if [ "$(NODE_OK)" != "yes" ]; then \
		echo "Error: Node.js 22+ required (found: $$(node -v 2>/dev/null || echo 'not installed'))"; \
		echo "Run: nvm use 24"; \
		exit 1; \
	fi

# Directories
PREPROCESSOR_DIR := preprocessor
THEME_DIR := quartz-theme
QUARTZ_DIR := quartz-build
EXAMPLE_DIR := example
OUTPUT_DIR := public

# Binaries
PREPROCESSOR := $(PREPROCESSOR_DIR)/target/release/logseq-to-quartz

.PHONY: all check-node build-preprocessor setup-quartz preprocess copy-theme build serve clean help example

# Default target
all: example

# Build the Rust preprocessor
build-preprocessor:
	@echo "Building preprocessor..."
	cd $(PREPROCESSOR_DIR) && cargo build --release

# Clone and setup Quartz (if not exists)
setup-quartz: check-node
	@if [ ! -d "$(QUARTZ_DIR)" ]; then \
		echo "Cloning Quartz..."; \
		git clone --depth 1 https://github.com/jackyzha0/quartz.git $(QUARTZ_DIR); \
		cd $(QUARTZ_DIR) && npm ci; \
	else \
		echo "Quartz already exists, skipping clone..."; \
	fi

# Run preprocessor on example graph
preprocess: build-preprocessor
	@echo "Preprocessing example graph..."
	$(PREPROCESSOR) \
		--input $(EXAMPLE_DIR) \
		--output $(QUARTZ_DIR)/content \
		--create-stubs \
		--verbose

# Copy theme customizations to Quartz
copy-theme:
	@echo "Copying theme customizations..."
	cp "$(THEME_DIR)/quartz.config.ts" $(QUARTZ_DIR)/
	cp "$(THEME_DIR)/quartz.layout.ts" $(QUARTZ_DIR)/
	cp "$(THEME_DIR)/styles/custom.scss" $(QUARTZ_DIR)/quartz/styles/
	cp "$(THEME_DIR)/path.ts" $(QUARTZ_DIR)/quartz/util/
	# Components
	cp $(THEME_DIR)/components/*.tsx $(QUARTZ_DIR)/quartz/components/
	cp "$(THEME_DIR)/components/components-index.ts" $(QUARTZ_DIR)/quartz/components/index.ts
	# Scripts and styles
	cp $(THEME_DIR)/scripts/*.ts $(QUARTZ_DIR)/quartz/components/scripts/
	cp $(THEME_DIR)/styles/*.scss $(QUARTZ_DIR)/quartz/components/styles/

# Apply site configuration from preprocessor output
apply-config:
	@if [ -f "$(QUARTZ_DIR)/content/_site_config.json" ]; then \
		PAGE_TITLE=$$(cat $(QUARTZ_DIR)/content/_site_config.json | jq -r '.page_title'); \
		echo "Applying site title: $$PAGE_TITLE"; \
		sed -i '' "s/pageTitle: \"Cyber\"/pageTitle: \"$$PAGE_TITLE\"/" $(QUARTZ_DIR)/quartz.config.ts 2>/dev/null || \
		sed -i "s/pageTitle: \"Cyber\"/pageTitle: \"$$PAGE_TITLE\"/" $(QUARTZ_DIR)/quartz.config.ts; \
	fi

# Build Quartz site
build: setup-quartz preprocess copy-theme apply-config
	@echo "Building Quartz site..."
	cd $(QUARTZ_DIR) && npx quartz build -o ../$(OUTPUT_DIR)
	@echo "\nSite built successfully!"
	@echo "Output: $(OUTPUT_DIR)/"

# Serve Quartz site for development (with hot reload)
serve: setup-quartz preprocess copy-theme apply-config
	@echo "Starting Quartz dev server..."
	cd $(QUARTZ_DIR) && npx quartz build --serve

# Build the example site (alias)
example: build
	@echo "\nExample site ready at $(OUTPUT_DIR)/"

# Clean generated files
clean:
	@echo "Cleaning..."
	rm -rf $(QUARTZ_DIR)
	rm -rf $(OUTPUT_DIR)
	cd $(PREPROCESSOR_DIR) && cargo clean

# Clean only content (keep Quartz and preprocessor)
clean-content:
	rm -rf $(QUARTZ_DIR)/content
	rm -rf $(OUTPUT_DIR)

# Run preprocessor tests
test:
	cd $(PREPROCESSOR_DIR) && cargo test

# Watch mode - reprocess on changes (requires entr: brew install entr)
watch-content:
	@echo "Watching $(EXAMPLE_DIR) for changes... (Ctrl+C to stop)"
	@while true; do \
		find $(EXAMPLE_DIR) -name "*.md" -o -name "*.edn" | entr -d $(MAKE) reprocess; \
	done

# Watch preprocessor source and rebuild
watch-preprocessor:
	@echo "Watching preprocessor source... (Ctrl+C to stop)"
	@while true; do \
		find $(PREPROCESSOR_DIR)/src -name "*.rs" | entr -d $(MAKE) reprocess; \
	done

# Watch theme files and copy
watch-theme:
	@echo "Watching theme files... (Ctrl+C to stop)"
	@while true; do \
		find $(THEME_DIR) -type f | entr -d $(MAKE) copy-theme; \
	done

# Quick reprocess without rebuilding preprocessor (for content changes only)
reprocess:
	@echo "Reprocessing content..."
	@$(PREPROCESSOR) \
		--input $(EXAMPLE_DIR) \
		--output $(QUARTZ_DIR)/content \
		--create-stubs
	@echo "Done! Quartz will hot-reload automatically."

# Full dev mode: start server + watch all (run in separate terminals)
dev:
	@echo "Starting dev server with hot reload..."
	@echo ""
	@echo "Quartz watches content/ automatically."
	@echo "To also watch source files, run in separate terminals:"
	@echo "  make watch-content      # Watch example graph"
	@echo "  make watch-preprocessor # Watch Rust code"
	@echo "  make watch-theme        # Watch theme files"
	@echo ""
	@echo "Or install entr (brew install entr) for automatic rebuilds."
	@echo ""
	cd $(QUARTZ_DIR) && npx quartz build --serve

# Help
help:
	@echo "publish-quartz Makefile"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all / example    Build the example site (default)"
	@echo "  build            Full build: preprocess + build Quartz"
	@echo "  serve            Start dev server with hot reload"
	@echo "  dev              Start server (Quartz auto-reloads content)"
	@echo "  reprocess        Quick reprocess content only"
	@echo ""
	@echo "Watch modes (require: brew install entr):"
	@echo "  watch-content      Watch example/ and reprocess on change"
	@echo "  watch-preprocessor Watch Rust src/ and rebuild on change"
	@echo "  watch-theme        Watch theme/ and copy on change"
	@echo ""
	@echo "Other:"
	@echo "  build-preprocessor  Build the Rust preprocessor"
	@echo "  setup-quartz     Clone and setup Quartz framework"
	@echo "  preprocess       Run preprocessor on example graph"
	@echo "  copy-theme       Copy theme files to Quartz"
	@echo "  test             Run preprocessor tests"
	@echo "  clean            Remove all generated files"
	@echo "  clean-content    Remove content only (keep deps)"
	@echo "  help             Show this help message"
	@echo ""
	@echo "Custom graph:"
	@echo "  make build EXAMPLE_DIR=/path/to/your/logseq/graph"
