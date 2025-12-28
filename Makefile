.PHONY: help build build-all release release-all test bench lint fmt clean doc doc-pdf serve install all dist dist-compressed install-mdbook

# Binary name
BINARY := dx

# Features to enable by default
FEATURES := ui,egui

# Tool versions (keep in sync with CI)
MDBOOK_VERSION := 0.5.2

# Output directory for collected binaries
DIST_DIR := dist

# Cross-compilation targets
LINUX_TARGETS := x86_64-unknown-linux-gnu \
                 x86_64-unknown-linux-musl \
                 aarch64-unknown-linux-gnu

MACOS_TARGETS := x86_64-apple-darwin \
                 aarch64-apple-darwin

# Windows targets (gnullvm uses LLVM linker, no mingw-w64 needed)
WINDOWS_TARGETS := x86_64-pc-windows-gnullvm \
                   aarch64-pc-windows-gnullvm

ALL_TARGETS := $(LINUX_TARGETS) $(MACOS_TARGETS)

help:
	@echo "Available targets:"
	@echo "  build         - Build debug binary"
	@echo "  build-all     - Build debug for all platforms"
	@echo "  release       - Build release binary"
	@echo "  release-all   - Build release for all platforms"
	@echo "  dist          - Build all platforms and collect in dist/"
	@echo "  dist-compressed - Same as dist, but compress with UPX (Linux/Windows)"
	@echo "  test          - Run tests"
	@echo "  test-verbose  - Run tests with output"
	@echo "  bench         - Run benchmarks"
	@echo "  lint          - Format code and run clippy with autofix"
	@echo "  fmt           - Format code"
	@echo "  fmt-check     - Check formatting"
	@echo "  doc           - Build and open rustdoc"
	@echo "  doc-book      - Build mdbook (HTML + PDF)"
	@echo "  doc-pdf       - Build mdbook PDF only"
	@echo "  serve         - Serve mdbook locally"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install binary"
	@echo "  install-mdbook - Install mdbook (v$(MDBOOK_VERSION), same as CI)"
	@echo "  all           - Run lint, test, build"
	@echo "  ci            - Run CI checks"
	@echo ""
	@echo "Features enabled by default: $(FEATURES)"
	@echo "  ui   - TUI dashboard (ratatui/crossterm)"
	@echo "  egui - GUI demos (eframe/egui)"
	@echo ""
	@echo "Cross-compilation targets:"
	@echo "  Linux:   $(LINUX_TARGETS)"
	@echo "  macOS:   $(MACOS_TARGETS)"
	@echo "  Windows: $(WINDOWS_TARGETS)"

all: lint test build

# Build
build:
	cargo build --features $(FEATURES)

build-all: build-linux build-macos build-windows

build-linux:
	@for target in $(LINUX_TARGETS); do \
		echo "=== Building for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo zigbuild --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

build-macos:
	@for target in $(MACOS_TARGETS); do \
		echo "=== Building for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo build --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

build-windows:
	@for target in $(WINDOWS_TARGETS); do \
		echo "=== Building for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo zigbuild --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

release:
	cargo build --release --features $(FEATURES)

release-all: release-linux release-macos release-windows

release-linux:
	@for target in $(LINUX_TARGETS); do \
		echo "=== Building release for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo zigbuild --release --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

release-macos:
	@for target in $(MACOS_TARGETS); do \
		echo "=== Building release for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo build --release --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

release-windows:
	@for target in $(WINDOWS_TARGETS); do \
		echo "=== Building release for $$target ==="; \
		rustup target add $$target 2>/dev/null || true; \
		cargo zigbuild --release --features $(FEATURES) --target $$target || echo "Failed: $$target"; \
	done

# Distribute - collect all binaries into dist/
dist: release-all
	@mkdir -p $(DIST_DIR)
	@for target in $(LINUX_TARGETS) $(MACOS_TARGETS); do \
		if [ -f target/$$target/release/$(BINARY) ]; then \
			cp target/$$target/release/$(BINARY) $(DIST_DIR)/$(BINARY)-$$target; \
			echo "Copied $(BINARY)-$$target"; \
		fi; \
	done
	@for target in $(WINDOWS_TARGETS); do \
		if [ -f target/$$target/release/$(BINARY).exe ]; then \
			cp target/$$target/release/$(BINARY).exe $(DIST_DIR)/$(BINARY)-$$target.exe; \
			echo "Copied $(BINARY)-$$target.exe"; \
		fi; \
	done
	@echo ""
	@echo "All binaries collected in $(DIST_DIR)/"
	@ls -lh $(DIST_DIR)/

# Distribute with UPX compression (Linux/Windows only, macOS breaks with UPX)
dist-compressed: dist
	@if command -v upx >/dev/null 2>&1; then \
		echo ""; \
		echo "=== Compressing with UPX (Linux/Windows only) ==="; \
		for target in $(LINUX_TARGETS); do \
			if [ -f $(DIST_DIR)/$(BINARY)-$$target ]; then \
				echo "Compressing $(BINARY)-$$target..."; \
				upx --best -q $(DIST_DIR)/$(BINARY)-$$target || echo "  Failed to compress"; \
			fi; \
		done; \
		for target in $(WINDOWS_TARGETS); do \
			if [ -f $(DIST_DIR)/$(BINARY)-$$target.exe ]; then \
				echo "Compressing $(BINARY)-$$target.exe..."; \
				upx --best -q $(DIST_DIR)/$(BINARY)-$$target.exe || echo "  Failed to compress"; \
			fi; \
		done; \
		echo ""; \
		echo "=== Compressed binaries ==="; \
		ls -lh $(DIST_DIR)/; \
	else \
		echo ""; \
		echo "WARNING: UPX not found. Install with: brew install upx"; \
		echo "Binaries in $(DIST_DIR)/ are not compressed."; \
	fi

# Test
test:
	cargo test --features $(FEATURES)

test-verbose:
	cargo test --features $(FEATURES) -- --nocapture

# Benchmark
bench:
	cargo bench

# Lint and format
lint:
	cargo fmt
	cargo clippy --features $(FEATURES) --fix --allow-dirty --allow-staged -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

# Documentation
doc:
	cargo doc --features $(FEATURES) --no-deps --open

doc-book:
	mdbook build
	@if [ -f site/pdf/output.pdf ]; then \
		echo "PDF generated: site/pdf/output.pdf"; \
	fi

doc-pdf:
	@if ! command -v mdbook-pdf >/dev/null 2>&1; then \
		echo "mdbook-pdf not installed. Run: cargo install mdbook-pdf"; \
		exit 1; \
	fi
	mdbook build
	@echo "PDF: site/pdf/output.pdf"

serve:
	mdbook serve --open

# Clean
clean:
	cargo clean
	rm -rf site/
	rm -rf $(DIST_DIR)/

# Install
install:
	cargo install --path . --features $(FEATURES)

# Install mdbook (same version as CI)
install-mdbook:
	@echo "Installing mdbook v$(MDBOOK_VERSION)..."
	@if [ "$$(uname)" = "Darwin" ]; then \
		if [ "$$(uname -m)" = "arm64" ]; then \
			curl -sSL https://github.com/rust-lang/mdBook/releases/download/v$(MDBOOK_VERSION)/mdbook-v$(MDBOOK_VERSION)-aarch64-apple-darwin.tar.gz | tar -xz -C /usr/local/bin; \
		else \
			curl -sSL https://github.com/rust-lang/mdBook/releases/download/v$(MDBOOK_VERSION)/mdbook-v$(MDBOOK_VERSION)-x86_64-apple-darwin.tar.gz | tar -xz -C /usr/local/bin; \
		fi; \
	else \
		curl -sSL https://github.com/rust-lang/mdBook/releases/download/v$(MDBOOK_VERSION)/mdbook-v$(MDBOOK_VERSION)-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin; \
	fi
	@echo "Installed: $$(mdbook --version)"

# Check everything (CI)
ci: fmt-check lint test
	cargo build --release --features $(FEATURES)

