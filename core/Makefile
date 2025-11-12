# ═══════════════════════════════════════════════════════════════
# PIXLY Unified Build System v3.0.0
# GO + Rust Architecture
# ═══════════════════════════════════════════════════════════════

.PHONY: all cli services plugin clean test help install deps

# 默认目标
all: services cli

# ═══════════════════════════════════════════════════════════════
# Services (Backend)
# ═══════════════════════════════════════════════════════════════

services: go-ai rust-service

go-ai:
	@echo "🔨 Building GO AI Service..."
	cd cmd/ai-service && go build -o ../../bin/ai-service
	@echo "✅ GO AI Service built: bin/ai-service"

rust-service:
	@echo "🔨 Building Rust Service..."
	cd pixly-rust && cargo build --release
	cd cmd/rust-service && go build -o ../../bin/rust-service
	@echo "✅ Rust Service built: bin/rust-service"

# ═══════════════════════════════════════════════════════════════
# CLI Tool (Unified)
# ═══════════════════════════════════════════════════════════════

cli:
	@echo "🔨 Building PIXLY CLI..."
	cd cmd/pixly-cli && go build -o ../../bin/pixly
	@echo "✅ PIXLY CLI built: bin/pixly"
	@echo ""
	@echo "Usage:"
	@echo "  ./bin/pixly convert input.jpg output.avif"
	@echo "  ./bin/pixly health"
	@echo "  ./bin/pixly version"

# ═══════════════════════════════════════════════════════════════
# Plugin (Eagle Integration)
# ═══════════════════════════════════════════════════════════════

plugin-info:
	@echo "📦 Eagle Plugin Information"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "Plugin location: plugin_v3/"
	@echo "Preview plugin:  plugin_preview/"
	@echo ""
	@echo "Installation:"
	@echo "  1. Copy plugin_v3 to Eagle plugins folder"
	@echo "  2. Copy plugin_preview to Eagle plugins folder"
	@echo "  3. Restart Eagle"
	@echo ""
	@echo "Note: Plugin requires GO AI Service and Rust Service running"

# ═══════════════════════════════════════════════════════════════
# Dependencies
# ═══════════════════════════════════════════════════════════════

deps:
	@echo "📦 Installing dependencies..."
	go mod download
	cd pixly-rust && cargo fetch
	@echo "✅ Dependencies installed"

install-cobra:
	@echo "📦 Installing Cobra CLI..."
	go install github.com/spf13/cobra@latest
	@echo "✅ Cobra installed"

install-tools:
	@echo "📦 Installing required tools..."
	@echo "Note: This requires Homebrew on macOS"
	brew install ffmpeg libavif jpeg-xl webp exiftool tag
	@echo "✅ Tools installed"

# ═══════════════════════════════════════════════════════════════
# Testing
# ═══════════════════════════════════════════════════════════════

test:
	@echo "🧪 Running tests..."
	go test ./... -v
	cd pixly-rust && cargo test
	@echo "✅ Tests passed"

test-cli:
	@echo "🧪 Testing CLI..."
	./bin/pixly health
	@echo "✅ CLI test passed"

# ═══════════════════════════════════════════════════════════════
# Cleanup
# ═══════════════════════════════════════════════════════════════

clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf bin/
	cd pixly-rust && cargo clean
	@echo "✅ Cleaned"

clean-all: clean
	@echo "🧹 Removing deprecated tools..."
	rm -rf deprecated/standalone_tools
	@echo "✅ All cleaned"

# ═══════════════════════════════════════════════════════════════
# Development
# ═══════════════════════════════════════════════════════════════

dev-services:
	@echo "🚀 Starting development services..."
	@echo "Starting GO AI Service..."
	./bin/ai-service &
	@echo "Starting Rust Service..."
	./bin/rust-service &
	@echo "✅ Services started"

stop-services:
	@echo "🛑 Stopping services..."
	pkill -f ai-service
	pkill -f rust-service
	@echo "✅ Services stopped"

# ═══════════════════════════════════════════════════════════════
# Documentation
# ═══════════════════════════════════════════════════════════════

help:
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════╗"
	@echo "║                                                              ║"
	@echo "║     PIXLY Build System v3.0.0                               ║"
	@echo "║     GO + Rust Architecture                                  ║"
	@echo "║                                                              ║"
	@echo "╚══════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "Available targets:"
	@echo ""
	@echo "  🔨 Build:"
	@echo "     make all          - Build services and CLI"
	@echo "     make services     - Build GO AI + Rust services"
	@echo "     make cli          - Build unified CLI tool"
	@echo ""
	@echo "  📦 Dependencies:"
	@echo "     make deps         - Install Go/Rust dependencies"
	@echo "     make install-tools - Install system tools (ffmpeg, etc.)"
	@echo ""
	@echo "  🧪 Testing:"
	@echo "     make test         - Run all tests"
	@echo "     make test-cli     - Test CLI tool"
	@echo ""
	@echo "  �� Development:"
	@echo "     make dev-services - Start services for development"
	@echo "     make stop-services - Stop running services"
	@echo ""
	@echo "  🧹 Cleanup:"
	@echo "     make clean        - Clean build artifacts"
	@echo "     make clean-all    - Clean everything + deprecated"
	@echo ""
	@echo "  📖 Information:"
	@echo "     make plugin-info  - Show Eagle plugin information"
	@echo "     make help         - Show this help message"
	@echo ""

# ═══════════════════════════════════════════════════════════════
# Architecture Overview
# ═══════════════════════════════════════════════════════════════

arch:
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════╗"
	@echo "║  PIXLY Architecture v3.0.0                                   ║"
	@echo "╚══════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "┌─────────────────────────────────────────────────────────────┐"
	@echo "│  Frontend Layer                                             │"
	@echo "├─────────────────────────────────────────────────────────────┤"
	@echo "│  • Eagle Plugin (GUI)        - plugin_v3/                   │"
	@echo "│  • PIXLY CLI/TUI             - cmd/pixly-cli/               │"
	@echo "└─────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─────────────────────────────────────────────────────────────┐"
	@echo "│  Backend Services                                           │"
	@echo "├─────────────────────────────────────────────────────────────┤"
	@echo "│  • GO AI Service (Smart)     - cmd/ai-service/              │"
	@echo "│    - LightGBM models                                        │"
	@echo "│    - Parameter prediction                                   │"
	@echo "│                                                             │"
	@echo "│  • Rust Converter (Manual)   - pixly-rust/                 │"
	@echo "│    - High-performance conversion                            │"
	@echo "│    - AVIF/JXL/WebP support                                  │"
	@echo "└─────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─────────────────────────────────────────────────────────────┐"
	@echo "│  Shared Libraries                                           │"
	@echo "├─────────────────────────────────────────────────────────────┤"
	@echo "│  • pkg/metadata/             - Metadata preservation        │"
	@echo "│  • pkg/converter/            - Conversion logic             │"
	@echo "│  • pkg/system/               - System utilities             │"
	@echo "└─────────────────────────────────────────────────────────────┘"
	@echo ""

.DEFAULT_GOAL := help

# ===========================================
# Phase 13: Native AVIF Performance Testing
# ===========================================

.PHONY: test-native-avif
test-native-avif: build-rust
	@echo "$(CYAN)Running native AVIF performance tests...$(NC)"
	./test-native-avif.sh
	@echo "$(GREEN)✓ Native AVIF tests complete$(NC)"

.PHONY: benchmark-avif
benchmark-avif: build-rust
	@echo "$(CYAN)Running AVIF encoder benchmarks...$(NC)"
	@echo "$(YELLOW)Creating benchmark test image...$(NC)"
	@python3 -c "from PIL import Image; Image.new('RGB', (1920, 1080), (128, 128, 128)).save('bench-test.png')"
	@echo "$(YELLOW)Testing native AVIF encoder...$(NC)"
	@time pixly-rust/target/release/pixly-rust convert bench-test.png bench-native.avif --quality 85 --speed 4
	@echo "$(GREEN)✓ Benchmark complete$(NC)"
	@rm -f bench-test.png bench-native.avif

