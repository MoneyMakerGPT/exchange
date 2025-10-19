# Engine 模块测试说明

## 测试文件说明

我们为 [engine.rs](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/src/engine/engine.rs) 文件创建了两个测试文件：

1. [engine_test.rs](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/tests/engine_test.rs) - 完整测试用例
2. [simple_engine_test.rs](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/tests/simple_engine_test.rs) - 简化测试用例

## 测试内容

测试涵盖了 Engine 类的主要功能：

1. **Engine 创建测试**
   - 验证 Engine 实例能否正确创建
   - 检查初始状态是否正确（orderbooks 和 balances 应为空）

2. **用户余额初始化测试**
   - 测试 [init_user_balance](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/src/engine/engine.rs#L81-L120) 方法
   - 验证是否能正确为用户初始化 USDC 和 SOL 余额

3. **资金检查和锁定测试**
   - 测试 [check_and_lock_funds](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/src/engine/engine.rs#L403-L482) 方法
   - 验证买单和卖单的资金检查逻辑
   - 测试资金不足情况下的错误处理

4. **余额更新测试**
   - 测试 [update_balance_with_lock](file:///Users/guoyingcheng/claude_pro/exchange/crates/engine/src/engine/engine.rs#L537-L554) 方法
   - 验证能否正确更新用户余额

## 运行测试

由于当前环境存在网络连接问题，无法下载依赖包，因此无法直接运行测试。在正常网络环境下，可以使用以下命令运行测试：

```bash
# 进入项目根目录
cd /Users/guoyingcheng/claude_pro/exchange

# 运行所有测试
cargo test

# 运行 engine 模块测试
cargo test engine_test

# 运行简化测试
cargo test simple_engine_test
```

## 测试依赖

测试依赖于以下 crate：
- rust_decimal
- rust_decimal_macros
- 其他项目内部模块

## 网络问题解决

如果遇到网络连接问题，可以尝试以下解决方案：
1. 检查网络连接是否正常
2. 配置代理（如果需要）
3. 确保 Cargo 能够访问 crates.io
4. 如果已有依赖包，可以使用 `--offline` 参数运行测试