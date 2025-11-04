# Sophia Keyboard Sender

[English](README.en.md) | 中文

一个功能强大的跨平台键盘事件发送器，支持窗口目标定位、延迟控制和命令解析。基于 Windows API 和 `keyboard-codes` crate 构建。

## 功能特性

- 🎯 **窗口目标定位** - 向特定窗口发送键盘事件
- ⏱️ **精确延迟控制** - 可配置按键按下和释放的延迟时间
- ⌨️ **完整键盘支持** - 支持标准键、功能键、组合键等
- 🌐 **Unicode 支持** - 支持任意字符输入
- 🔧 **双重发送模式** - 全局模拟和窗口消息两种发送方式
- 📝 **命令解析器** - 通过文本命令执行键盘操作
- 🚀 **高性能** - 基于原生 Windows API，低延迟高精度
- 🔌 **模块化设计** - 按需启用所需功能

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
sophia_keyboard_sender = { git = "https://github.com/ymc-github/sophia_keyboard_sender", branch = "main" }
```

### 特性开关

- `global` - 全局键盘模拟功能（默认启用）
- `window_target` - 窗口目标功能（默认启用）
- `command_parser` - 文本命令解析功能（默认启用）
- `convenience` - 便捷函数
- `full` - 启用所有功能

最小化配置：
```toml
sophia_keyboard_sender = { git = "...", default-features = false, features = ["global"] }
```

## 快速开始

### 基本用法

```rust
use sophia_keyboard_sender::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 发送单个按键
    key_click(Key::A, None)?;

    // 发送字符
    send_char('!')?;

    // 发送字符串
    type_string("Hello, World!")?;

    // 发送组合键 (Ctrl+C)
    press_combination(&[Modifier::Control], Key::C, None)?;

    Ok(())
}
```

### 命令解析器（推荐）

```rust
use sophia_keyboard_sender::send;

// 基本按键操作
send("key:a")?;                    // 按下并释放 A 键
send("key:enter")?;                // 按下回车键
send("char:!")?;                   // 发送感叹号
send("text:hello world")?;         // 输入 "hello world"

// 快捷键
send("shortcut:ctrl+c")?;          // Ctrl+C
send("shortcut:alt+f4")?;          // Alt+F4
send("shortcut:ctrl+shift+esc")?;  // Ctrl+Shift+Esc

// 高级选项
send("key:a,duration:100ms")?;     // 按住按键 100ms
send("key:a,hwnd:123456")?;        // 发送到特定窗口
send("text:test,hwnd:0x1A2B,duration:10ms")?;
```

### 窗口目标操作

```rust
use sophia_keyboard_sender::*;

// 向特定窗口发送按键
send_key_click_to_window(123456, Key::Enter, None)?;

// 向特定窗口输入文本
type_string_to_window(123456, "发送到特定窗口的文本")?;

// 控制窗口焦点
set_window_focus(123456, true)?;  // 置于顶层并获取焦点
```

### 便捷函数

```rust
use sophia_keyboard_sender::*;

send_tab()?;        // 发送 Tab 键
send_enter()?;      // 发送 Enter 键
send_escape()?;     // 发送 Escape 键
send_space()?;      // 发送空格键
send_backspace()?;  // 发送退格键
send_delete()?;     // 发送删除键
```

## API 参考

### 核心函数

#### 全局键盘模拟
- `key_down(key: Key)` - 按下按键
- `key_up(key: Key)` - 释放按键
- `key_click(key: Key, duration: Option<Duration>)` - 点击按键（可设置按下持续时间）
- `send_char(c: char)` - 发送字符（支持 Unicode）
- `type_string(text: &str)` - 输入字符串
- `press_combination(modifiers: &[Modifier], key: Key, duration: Option<Duration>)` - 发送组合键

#### 窗口目标操作
- `send_key_down_to_window(hwnd: WindowHandle, key: Key)` - 向窗口发送按键按下
- `send_key_up_to_window(hwnd: WindowHandle, key: Key)` - 向窗口发送按键释放
- `send_key_click_to_window(hwnd: WindowHandle, key: Key, duration: Option<Duration>)` - 向窗口发送按键点击
- `send_char_to_window(hwnd: WindowHandle, c: char)` - 向窗口发送字符
- `type_string_to_window(hwnd: WindowHandle, text: &str)` - 向窗口输入字符串
- `set_window_focus(hwnd: WindowHandle, bring_to_top: bool)` - 控制窗口焦点

#### 命令解析器
- `send(command: &str)` - 执行文本命令
- `shortcut(shortcut: &str)` - 发送键盘快捷键
- `parse_duration(duration_str: &str)` - 解析持续时间字符串
- `parse_key(key_str: &str)` - 解析键名
- `parse_modifier(modifier_str: &str)` - 解析修饰符名

### 支持的键名

#### 字母键
```rust
"a", "b", "c", ..., "z"  // 映射到 Key::A, Key::B 等
```

#### 数字键
```rust
"0", "1", "2", ..., "9"  // 映射到 Key::D0, Key::D1 等
```

#### 特殊键
```rust
"enter", "space", "tab", "escape", "backspace", "delete",
"insert", "home", "end", "pageup", "pagedown"
```

#### 方向键
```rust
"up", "down", "left", "right"  // 映射到 ArrowUp, ArrowDown 等
```

#### 功能键
```rust
"f1", "f2", ..., "f12"  // 映射到 F1, F2 等
```

#### 修饰键
```rust
"ctrl", "shift", "alt", "meta",
"leftctrl", "rightctrl", "leftshift", "rightshift",
"leftalt", "rightalt", "leftmeta", "rightmeta"
```

## 命令语法

### 基本格式
```
键:值,键2:值2,键3:值3
```

### 支持的命令

#### 按键操作
```rust
"key:a"                          // 点击 A 键
"key:enter,duration:100ms"       // 按住 Enter 键 100ms
"key:a,hwnd:123456"              // 发送到窗口 123456
```

#### 字符操作
```rust
"char:!"                         // 发送感叹号
"char:!,hwnd:0x1A2B"             // 发送到特定窗口
```

#### 文本操作
```rust
"text:hello world"               // 输入文本
"text:test,hwnd:123456"          // 向特定窗口输入文本
"text:hello,duration:10ms"       // 输入文本，字符间有延迟
```

#### 快捷键操作
```rust
"shortcut:ctrl+c"                // Ctrl+C
"shortcut:alt+tab"               // Alt+Tab
"shortcut:ctrl+shift+escape"     // Ctrl+Shift+Escape
```

#### 基于动作的格式
```rust
"action:key_click,key:a"         // 点击 A 键
"action:char,char:!"             // 发送字符
"action:text,text:hello"         // 输入文本
"action:key_down,key:shift"      // 按下 Shift 键
"action:key_up,key:shift"        // 释放 Shift 键
```

### 持续时间格式
- `"100ms"` - 100 毫秒
- `"2s"` - 2 秒
- `"500ms"` - 500 毫秒

### 窗口句柄格式
- `"123456"` - 十进制窗口句柄
- `"0x1A2B"` - 十六进制窗口句柄
- `"0"` 或 `""` - 当前前景窗口

## 使用场景

### 自动化测试
```rust
// 自动化表单填写
send("text:张三")?;
send("key:tab")?;
send("text:zhangsan@example.com")?;
send("key:tab")?;
send("text:password123")?;
send("key:enter")?;
```

### 游戏自动化
```rust
// 带定时的游戏控制
send("key:w,duration:200ms")?;    // 向前移动
send("key:space")?;               // 跳跃
send("shortcut:ctrl+1")?;         // 使用物品1
```

### 远程控制
```rust
// 控制远程应用程序
set_window_focus(remote_window, true)?;
send("text:git status")?;
send("key:enter")?;
```

### 宏录制
```rust
// 执行录制的宏
send("shortcut:win+r")?;
send("text:notepad")?;
send("key:enter,duration:500ms")?;
send("text:来自宏的问候!")?;
```

## 错误处理

所有函数都返回 `Result<T, KeyboardSenderError>`：

```rust
#[derive(Error, Debug)]
pub enum KeyboardSenderError {
    ParseError(String),           // 解析错误
    UnsupportedKey(String),       // 不支持的键
    UnsupportedModifier(String),  // 不支持的修饰符
    InvalidDuration(String),      // 无效的持续时间
    InvalidWindowHandle(String),  // 无效的窗口句柄
    CommandParseError(String),    // 命令解析错误
    FeatureNotEnabled(String),    // 特性未启用
    WindowsError,                 // Windows API 错误
}
```

## 平台支持

当前支持 **Windows** 平台。Linux 和 macOS 支持计划在未来的版本中提供。

## 许可证

本项目采用 MIT OR Apache-2.0 双许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 相关项目

- [keyboard-codes](https://github.com/ymc-github/keyboard-codes) - 跨平台键盘键码映射库
