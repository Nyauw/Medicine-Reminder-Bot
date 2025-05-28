#!/bin/bash

# Medicine Reminder Bot 启动脚本
# 用于在Linux上稳定运行机器人

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
}

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo 未安装，请先安装 Rust"
        exit 1
    fi
    
    if ! command -v systemctl &> /dev/null; then
        log_warn "systemctl 不可用，将使用简单模式运行"
    fi
}

# 检查环境变量
check_env() {
    log_info "检查环境变量..."
    
    if [ ! -f ".env" ]; then
        log_error ".env 文件不存在，请创建并配置以下变量："
        echo "TELOXIDE_TOKEN=your_bot_token"
        echo "CHAT_ID=your_chat_id"
        exit 1
    fi
    
    source .env
    
    if [ -z "$TELOXIDE_TOKEN" ]; then
        log_error "TELOXIDE_TOKEN 未设置"
        exit 1
    fi
    
    if [ -z "$CHAT_ID" ]; then
        log_error "CHAT_ID 未设置"
        exit 1
    fi
    
    log_info "环境变量检查通过"
}

# 编译程序
build_program() {
    log_info "编译程序..."
    
    # 设置环境变量以使用 rustls
    export CARGO_NET_GIT_FETCH_WITH_CLI=true
    
    if cargo build --release; then
        log_info "编译成功"
    else
        log_error "编译失败"
        exit 1
    fi
}

# 运行程序（带重启机制）
run_with_restart() {
    local max_restarts=10
    local restart_count=0
    local restart_delay=5
    
    log_info "启动程序（带自动重启机制）..."
    log_info "最大重启次数: $max_restarts"
    log_info "重启延迟: ${restart_delay}秒"
    
    while [ $restart_count -lt $max_restarts ]; do
        log_info "启动尝试 $((restart_count + 1))/$max_restarts"
        
        # 设置环境变量
        export RUST_LOG=info
        export RUST_BACKTRACE=1
        
        # 运行程序
        if timeout 3600 ./target/release/medicine-reminder; then
            log_info "程序正常退出"
            break
        else
            exit_code=$?
            if [ $exit_code -eq 124 ]; then
                log_warn "程序运行超时（1小时），重启..."
            else
                log_warn "程序异常退出，退出码: $exit_code"
            fi
            
            restart_count=$((restart_count + 1))
            
            if [ $restart_count -lt $max_restarts ]; then
                log_info "等待 ${restart_delay} 秒后重启..."
                sleep $restart_delay
                
                # 指数退避
                restart_delay=$((restart_delay * 2))
                if [ $restart_delay -gt 300 ]; then
                    restart_delay=300  # 最大5分钟
                fi
            fi
        fi
    done
    
    if [ $restart_count -eq $max_restarts ]; then
        log_error "达到最大重启次数，程序停止"
        exit 1
    fi
}

# 简单运行模式
run_simple() {
    log_info "启动程序（简单模式）..."
    
    export RUST_LOG=info
    export RUST_BACKTRACE=1
    
    exec ./target/release/medicine-reminder
}

# 创建 systemd 服务
create_systemd_service() {
    local service_name="medicine-reminder"
    local service_file="/etc/systemd/system/${service_name}.service"
    local current_dir=$(pwd)
    local current_user=$(whoami)
    
    log_info "创建 systemd 服务..."
    
    if [ "$EUID" -ne 0 ]; then
        log_error "需要 root 权限来创建 systemd 服务"
        log_info "请使用: sudo $0 install"
        exit 1
    fi
    
    cat > "$service_file" << EOF
[Unit]
Description=Medicine Reminder Bot
After=network.target
Wants=network.target

[Service]
Type=simple
User=$current_user
WorkingDirectory=$current_dir
ExecStart=$current_dir/target/release/medicine-reminder
Restart=always
RestartSec=10
Environment=RUST_LOG=info
Environment=RUST_BACKTRACE=1

# 安全设置
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$current_dir

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl enable "$service_name"
    
    log_info "systemd 服务已创建: $service_file"
    log_info "使用以下命令管理服务:"
    log_info "  启动: sudo systemctl start $service_name"
    log_info "  停止: sudo systemctl stop $service_name"
    log_info "  状态: sudo systemctl status $service_name"
    log_info "  日志: sudo journalctl -u $service_name -f"
}

# 主函数
main() {
    case "${1:-run}" in
        "build")
            check_dependencies
            build_program
            ;;
        "run")
            check_dependencies
            check_env
            build_program
            run_with_restart
            ;;
        "simple")
            check_dependencies
            check_env
            build_program
            run_simple
            ;;
        "install")
            check_dependencies
            check_env
            build_program
            create_systemd_service
            ;;
        "help"|"-h"|"--help")
            echo "用法: $0 [命令]"
            echo ""
            echo "命令:"
            echo "  build   - 仅编译程序"
            echo "  run     - 编译并运行程序（带重启机制，默认）"
            echo "  simple  - 编译并运行程序（简单模式）"
            echo "  install - 安装为 systemd 服务（需要 root 权限）"
            echo "  help    - 显示此帮助信息"
            ;;
        *)
            log_error "未知命令: $1"
            log_info "使用 '$0 help' 查看帮助"
            exit 1
            ;;
    esac
}

# 信号处理
trap 'log_info "收到中断信号，正在退出..."; exit 0' INT TERM

# 运行主函数
main "$@"
