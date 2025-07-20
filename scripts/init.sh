#!/bin/bash
# OmniAgent 初始化脚本

echo "🚀 初始化 OmniAgent 应用程序..."

# 设置环境变量
echo "📋 设置环境变量..."
cp .env.example .env

# 创建默认配置（如果不存在）
if [ ! -f "config.json" ]; then
    echo "📄 创建默认配置文件..."
    cp config.example.json config.json
fi

# 构建项目
echo "🔨 构建项目..."
cargo build --release

# 运行测试
echo "🧪 运行测试..."
cargo test

echo "✅ OmniAgent 初始化完成！"
echo ""
echo "📖 使用方法："
echo "1. 编辑 config.json 配置你的LLM提供商"
echo "2. 设置环境变量（见 .env 文件）"
echo "3. 运行: cargo run --release"
echo "4. 访问: http://localhost:8080/health"
echo ""
echo "📚 查看文档: docs/USER_GUIDE.md"