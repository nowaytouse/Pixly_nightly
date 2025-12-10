.PHONY: build deploy build-deploy clean test help

# 默认目标
all: build-deploy

# 编译 Rust 项目 (release)
build:
	@echo "🔨 编译 Rust 项目..."
	cargo build --release
	@echo "✅ 编译完成"

# 部署到 Eagle 插件
deploy:
	@echo "📦 部署到 Eagle 插件..."
	@./scripts/deploy-to-plugins.sh

# 编译并部署 (一键完成)
build-deploy: build deploy
	@echo ""
	@echo "🎉 编译和部署全部完成!"
	@echo "现在可以在 Eagle 中测试插件了"

# 清理编译产物
clean:
	@echo "🧹 清理编译产物..."
	cargo clean
	rm -f plugin/ai-vue-refactor/bin/pixly-converter
	rm -f plugin/format-vue/bin/pixly-converter
	@echo "✅ 清理完成"

# 运行测试
test:
	@echo "🧪 运行测试..."
	cargo test

# 快速部署 (使用已有的 release 二进制)
quick-deploy:
	@echo "⚡ 快速部署 (不重新编译)..."
	@./scripts/deploy-to-plugins.sh

# 重新编译并部署 (强制重新编译)
rebuild: clean build-deploy

# 显示帮助信息
help:
	@echo "Pixly 构建系统"
	@echo ""
	@echo "可用命令:"
	@echo "  make build          - 编译 Rust 项目 (release)"
	@echo "  make deploy         - 部署到 Eagle 插件"
	@echo "  make build-deploy   - 编译并部署 (默认)"
	@echo "  make quick-deploy   - 快速部署 (不重新编译)"
	@echo "  make clean          - 清理编译产物"
	@echo "  make test           - 运行测试"
	@echo "  make rebuild        - 清理并重新编译"
	@echo "  make help           - 显示此帮助信息"
