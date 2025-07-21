#!/bin/bash

# OmniAgent 测试脚本
# 用于测试智能体应用的所有功能

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
SERVER_URL="http://localhost:8080"
MAX_WAIT_TIME=30

# 打印信息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
print_info "检查依赖..."
command -v curl >/dev/null 2>&1 || { print_error "curl 未安装"; exit 1; }
command -v jq >/dev/null 2>&1 || { print_warning "jq 未安装，输出将不格式化"; }

# 启动应用
print_info "启动 OmniAgent..."
if pgrep -f "omni-agent" > /dev/null; then
    print_warning "应用已在运行，跳过启动"
else
    cargo build --quiet
    cargo run -- --mock --port 8080 > agent.log 2>&1 &
    AGENT_PID=$!
    print_info "应用 PID: $AGENT_PID"
fi

# 等待应用启动
print_info "等待应用启动..."
wait_count=0
while [ $wait_count -lt $MAX_WAIT_TIME ]; do
    if curl -s "$SERVER_URL/health" > /dev/null 2>&1; then
        print_success "应用已启动"
        break
    fi
    sleep 1
    ((wait_count++))
done

if [ $wait_count -eq $MAX_WAIT_TIME ]; then
    print_error "应用启动超时"
    exit 1
fi

# 测试结果文件
RESULT_FILE="/tmp/omniagent_test_result.json"
echo '{"tests": [], "summary": {"passed": 0, "failed": 0, "total": 0}}' > "$RESULT_FILE"

# 测试函数
run_test() {
    local test_name="$1"
    local test_description="$2"
    local command="$3"
    local expected_pattern="$4"
    
    print_info "运行测试: $test_description"
    
    if eval "$command" > "/tmp/omniagent_${test_name}.log" 2>&1; then
        response=$(cat "/tmp/omniagent_${test_name}.log")
        if echo "$response" | grep -q "$expected_pattern"; then
            print_success "$test_name: 通过"
            
            if command -v jq >/dev/null 2>&1; then
                echo "$response" | jq '.' 2>/dev/null || echo "$response"
            else
                echo "$response"
            fi
            
            # 更新测试结果
            jq --arg name "$test_name" --arg desc "$test_description" \
               '.tests += [{"name": $name, "description": $desc, "status": "passed"}] | .summary.passed += 1 | .summary.total += 1' \
               "$RESULT_FILE" > "$RESULT_FILE.tmp" && mv "$RESULT_FILE.tmp" "$RESULT_FILE"
        else
            print_error "$test_name: 失败 - 未找到预期内容"
            echo "响应: $response"
            
            jq --arg name "$test_name" --arg desc "$test_description" \
               '.tests += [{"name": $name, "description": $desc, "status": "failed"}] | .summary.failed += 1 | .summary.total += 1' \
               "$RESULT_FILE" > "$RESULT_FILE.tmp" && mv "$RESULT_FILE.tmp" "$RESULT_FILE"
        fi
    else
        print_error "$test_name: 失败 - 命令执行错误"
        
        jq --arg name "$test_name" --arg desc "$test_description" \
           '.tests += [{"name": $name, "description": $desc, "status": "failed"}] | .summary.failed += 1 | .summary.total += 1' \
           "$RESULT_FILE" > "$RESULT_FILE.tmp" && mv "$RESULT_FILE.tmp" "$RESULT_FILE"
    fi
    echo "----------------------------------------"
}

# 测试1: 健康检查
run_test "health_check" "健康检查" \
    "curl -s '$SERVER_URL/health'" \
    "healthy"

# 测试2: 智能体信息
run_test "agent_info" "智能体信息" \
    "curl -s '$SERVER_URL/info'" \
    "omni-agent"

# 测试3: MCP工具测试 - 文件操作
run_test "mcp_file_test" "MCP文件工具测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"请帮我读取文件\", \"context\": {}}'" \
    "mcp_tool"

# 测试4: A2A智能体测试 - 天气查询
run_test "a2a_weather_test" "A2A天气智能体测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"北京天气如何\", \"context\": {}}'" \
    "a2a_agent"

# 测试5: 本地LLM测试 - 通用问题
run_test "local_llm_test" "本地LLM测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"解释一下量子计算\", \"context\": {}}'" \
    "local_llm"

# 测试6: 计算器测试
run_test "calculator_test" "计算器工具测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"计算1+1\", \"context\": {}}'" \
    "mcp_tool"

# 测试7: 新闻测试
run_test "news_test" "新闻智能体测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"今天有什么新闻\", \"context\": {}}'" \
    "a2a_agent"

# 测试8: 复杂问题测试
run_test "complex_test" "复杂问题测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"如果太阳消失了，地球会怎样\", \"context\": {}}'" \
    "local_llm"

# 测试9: 空消息测试
run_test "empty_message_test" "空消息测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"\", \"context\": {}}'" \
    "消息"

# 测试10: 特殊字符测试
run_test "special_chars_test" "特殊字符测试" \
    "curl -s -X POST '$SERVER_URL/chat' -H 'Content-Type: application/json' -d '{\"message\": \"你好！今天天气怎么样？@#$%^&*()\", \"context\": {}}'" \
    "天气"

# 测试结果汇总
print_info "测试完成，生成报告..."
echo "======================================"
echo "测试报告："
echo "======================================"

if command -v jq >/dev/null 2>&1; then
    cat "$RESULT_FILE" | jq '.'
else
    cat "$RESULT_FILE"
fi

# 清理
rm -f /tmp/omniagent_*.log
rm -f "$RESULT_FILE"

# 停止应用
if [ -n "$AGENT_PID" ]; then
    print_info "停止应用..."
    kill $AGENT_PID 2>/dev/null || true
fi

# 总结
print_info "测试完成！"
print_info "查看详细日志: tail -f agent.log"

# 退出码
total_tests=$(jq -r '.summary.total' "$RESULT_FILE" 2>/dev/null || echo 0)
passed_tests=$(jq -r '.summary.passed' "$RESULT_FILE" 2>/dev/null || echo 0)

if [ "$passed_tests" -eq "$total_tests" ]; then
    print_success "所有测试通过！"
    exit 0
else
    print_error "部分测试失败"
    exit 1
fi