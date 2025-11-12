SHELL := /bin/bash

.PHONY: all build fmt tidy test-mini release clean

all: build

fmt:
	@echo "==> go fmt"
	@go fmt ./...

tidy:
	@echo "==> go mod tidy"
	@go mod tidy

build: fmt tidy
	@echo "==> building tools"
	@echo "Building static2avif..."
	@cd static2avif && go build -o ../bin/static2avif .
	@echo "Building static2jxl..."
	@cd static2jxl && go build -o ../bin/static2jxl .
	@echo "Building dynamic2avif..."
	@cd dynamic2avif && go build -o ../bin/dynamic2avif .
	@echo "Building dynamic2jxl..."
	@cd dynamic2jxl && go build -o ../bin/dynamic2jxl .

test-mini: build
	@echo "==> running mini test (dry-run)"
	@echo "Mini test not applicable to standalone tools"
	@echo "Run individual tools for testing: ./bin/static2avif, ./bin/static2jxl, etc."

release: build
	@echo "==> release artifacts in ./bin"

clean:
	@rm -rf ./bin/*
	@find . -name "*.log" -delete


