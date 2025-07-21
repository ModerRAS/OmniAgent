FROM rust:1.88-slim AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 复制所有文件
COPY . .

# 构建应用
RUN cargo build --release

FROM debian:bookworm-slim

# 安装必要的依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN groupadd -r omniagent && useradd -r -g omniagent omniagent

# 创建应用目录
RUN mkdir -p /app/data && chown -R omniagent:omniagent /app

WORKDIR /app

# 复制二进制文件
COPY --from=builder /app/target/release/omni-agent /usr/local/bin/

# 复制配置文件
COPY --chown=omniagent:omniagent config.example.json /app/config.json
COPY --chown=omniagent:omniagent .env.example /app/.env.example

USER omniagent

EXPOSE 8080

ENV PORT=8080
ENV HOST=0.0.0.0
ENV RUST_LOG=info

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

CMD ["omni-agent"]
