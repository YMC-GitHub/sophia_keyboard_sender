# Sophia Keyboard Sender

[English](README.en.md) | 中文

一个支持窗口目标定位和延迟控制的键盘事件发送器，基于 Windows API 和 `keyboard-codes` crate 构建。

## 功能特性

- 🎯 **窗口目标定位** - 支持向指定窗口发送键盘事件
- ⏱️ **精确延迟控制** - 可配置按键按下和释放的延迟时间
- ⌨️ **完整键盘支持** - 支持标准键、功能键、组合键等
- 🌐 **Unicode 支持** - 支持任意字符输入
- 🔧 **两种发送模式** - 全局模拟和窗口消息两种发送方式
- 🚀 **高性能** - 基于原生 Windows API，低延迟高精度

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
sophia_keyboard_sender = { git = "https://github.com/ymc-github/sophia_keyboard_sender", branch = "main" }
```

## 快速开始

### 基本用法

```rust
use sophia_keyboard_sender::*;
use keyboard_codes::{Key, Modifier};
use std::time::Duration;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 发送单个按键
    let key_a = Key::from_str("A").unwrap();
    key_click(key_a, None)?;

    // 发送字符
    send_char('!')?;

    // 发送字符串
    type_string("Hello, World!")?;

    // 发送组合键 (Ctrl+C)
    let ctrl = Modifier::from_str("Control").unwrap();
    let key_c = Key::from_str("C").unwrap();
    press_combination(&[ctrl], key_c, None)?;

    Ok(())
}
```

### 带延迟的按键操作

```rust
// 按键按下 100ms 后释放
key_click(Key::from_str("Enter").unwrap(), Some(Duration::from_millis(100)))?;

// 组合键保持按下状态 500ms
press_combination(
    &[Modifier::from_str("Control").unwrap()], 
    Key::from_str("V").unwrap(), 
    Some(Duration::from_millis(500))
)?;
```

### 向指定窗口发送事件

```rust
use windows::Win32::Foundation::HWND;

// 假设你有一个窗口句柄
let hwnd = HWND(0x123456); // 替换为实际的窗口句柄

// 激活窗口
set_window_focus(hwnd, true)?;

// 向指定窗口发送按键
send_key_click_to_window(hwnd, Key::from_str("Enter").unwrap(), None)?;

// 向指定窗口输入文本
type_string_to_window(hwnd, "文本输入到指定窗口")?;
```

## API 参考

### 全局键盘事件（模拟物理键盘）

- `key_down(key: Key)` - 按下按键
- `key_up(key: Key)` - 释放按键  
- `key_click(key: Key, duration: Option<Duration>)` - 点击按键（可设置按下持续时间）
- `send_char(c: char)` - 发送字符（支持 Unicode）
- `type_string(text: &str)` - 输入字符串
- `press_combination(modifiers: &[Modifier], key: Key, duration: Option<Duration>)` - 发送组合键

### 窗口目标键盘事件（通过窗口消息）

- `send_key_down_to_window(hwnd: HWND, key: Key)` - 向窗口发送按键按下
- `send_key_up_to_window(hwnd: HWND, key: Key)` - 向窗口发送按键释放
- `send_key_click_to_window(hwnd: HWND, key: Key, duration: Option<Duration>)` - 向窗口发送按键点击
- `send_char_to_window(hwnd: HWND, c: char)` - 向窗口发送字符
- `type_string_to_window(hwnd: HWND, text: &str)` - 向窗口输入字符串
- `set_window_focus(hwnd: HWND, bring_to_top: bool)` - 控制窗口焦点

## 键盘键位支持

本库通过 `keyboard-codes` crate 支持完整的键盘键位：

### 字母键
```rust
Key::from_str("A").unwrap()  // A 键
Key::from_str("Z").unwrap()  // Z 键
```

### 数字键
```rust
Key::from_str("D0").unwrap() // 主键盘区 0
Key::from_str("Num0").unwrap() // 小键盘 0
```

### 功能键
```rust
Key::from_str("F1").unwrap()  // F1 键
Key::from_str("F12").unwrap() // F12 键
```

### 控制键
```rust
Key::from_str("Enter").unwrap()    // 回车键
Key::from_str("Escape").unwrap()   // ESC 键
Key::from_str("Space").unwrap()    // 空格键
Key::from_str("Tab").unwrap()      // Tab 键
```

### 方向键
```rust
Key::from_str("ArrowUp").unwrap()    // 上箭头
Key::from_str("ArrowDown").unwrap()  // 下箭头
Key::from_str("ArrowLeft").unwrap()  // 左箭头  
Key::from_str("ArrowRight").unwrap() // 右箭头
```

### 修饰键
```rust
Modifier::from_str("Shift").unwrap()     // Shift 键
Modifier::from_str("Control").unwrap()   // Ctrl 键
Modifier::from_str("Alt").unwrap()       // Alt 键
Modifier::from_str("Meta").unwrap()      // Windows 键
```

## 使用场景

### 自动化测试
```rust
// 自动化填写表单
type_string("测试用户")?;
key_click(Key::from_str("Tab").unwrap(), None)?;
type_string("test@example.com")?;
key_click(Key::from_str("Enter").unwrap(), None)?;
```

### 游戏自动化
```rust
// 游戏快捷键
press_combination(
    &[Modifier::from_str("Control").unwrap()],
    Key::from_str("Q").unwrap(),
    Some(Duration::from_millis(100))
)?;
```

### 远程控制
```rust
// 向远程窗口发送控制命令
set_window_focus(remote_window, true)?;
type_string_to_window(remote_window, "执行命令")?;
key_click_to_window(remote_window, Key::from_str("Enter").unwrap(), None)?;
```

## 注意事项

1. **管理员权限**：某些操作可能需要管理员权限
2. **窗口焦点**：向指定窗口发送消息时，确保窗口存在且可接收消息
3. **延迟设置**：合理的延迟设置可以提高操作可靠性
4. **错误处理**：所有函数都返回 `Result`，建议进行错误处理

## 许可证

本项目采用 MIT OR Apache-2.0 双许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 相关项目

- [keyboard-codes](https://github.com/ymc-github/keyboard-codes) - 跨平台键盘键码映射库