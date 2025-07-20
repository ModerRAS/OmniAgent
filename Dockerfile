FROM rust:1.80-slim as builder

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟项目以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制实际源代码
COPY src ./src
COPY config.example.json ./

# 构建实际应用
RUN touch src/main.rs && \
    cargo build --release

FROM debian:bookworm-slim

# 安装必要的依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN groupadd -r omniagent && useradd -r -g omniagent omniagent

# 创建应用目录
RUN mkdir -p /app/data && chown -R omniagent:omniagent /app

WORKDIR /app

# 复制二进制文件
COPY --from=builder /app/target/release/omni-agent /usr/local/bin/

# 复制默认配置
COPY --from=builder /app/config.example.json /app/config.json
COPY --from=builder /app/.env.example /app/.env.example

# 设置权限
RUN chown -R omniagent:omniagent /app

USER omniagent

EXPOSE 8080

ENV PORT=8080
ENV HOST=0.0.0.0
ENV RUST_LOG=info

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

CMD ["omni-agent"]