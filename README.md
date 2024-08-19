# PadReticle

<span style="font-size: 1.5rem;color: #00ffd1;">Better Crosshairs for Controller Players
</span>

[中文](#简介)
[English](#Introduction)


## Introduction

**PadReticle** is a tool designed to enhance the gaming experience for controller players by providing customizable crosshairs.

---

## Features


- Optimized for **controllers**; crosshairs can be **hidden** while aiming.
- Styles are **customizable**.
- **Lightweight**, with strong **backend extensibility** ([backend crate](https://crates.io/crates/xci)) (~~frontend is fuking chaotic~~).



---

## Getting Started

### Installation
1. Download the latest version of PadReticle from the repository.
2. Run the installer `PadReticle_x.x.x_x64_en-US.msi`.
3. **Choose a suitable installation directory** that you can easily find.
4. Prepare a [crosshair PNG](https://crosshair.themeta.gg/) and place it in the **installation directory** (where `PadReticle.exe` is located).
5. Rename the PNG file to `crosshair.png`.
6. Click `PadReticle.exe`.

### Settings
Configure your preferences by creating a `config.toml` file:
```
.
├── config.toml
├── crosshair.png
├── PadReticle.exe
└── Uninstall PadReticle.lnk
```
Example `config.toml`:
```toml
aim_key = 8
polling_interval = 80
```
See [aim_key list](#aim-key-list) for more details.

---

## Maintenance
We welcome your feedback and encourage you to open an issue if you have any suggestions or encounter any problems.

---

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
---

## 简介

**PadReticle** 

基于Tauri，**手把玩家的准心工具**。

---

## 特性

- 针对 **手把**, 在瞄准时可以 **隐藏** 准心。
- **自定义程度高**。
- **足够轻量**，[后端](https://crates.io/crates/xci)**可拓展性强**(~~前端很混沌~~)。

---

## 开始使用

### 安装
1. 从仓库中下载最新版本的 PadReticle。
2. 运行安装程序 `PadReticle_x.x.x_x64_en-US.msi`。
3. **选择一个你可以轻松找到的安装目录**。
4. (**必须**)准备一个 [准心 PNG 文件](https://crosshair.themeta.gg/) 并将其放置在 **安装目录** 中（与 `PadReticle.exe` 文件相同的目录）。
5. 将 PNG 文件重命名为 `crosshair.png`。
6. 点击`PadReticle.exe`

### 使用

1. 两盏灯分别检测控制器是否连接，开镜键是否被按下（插线或者蓝牙都可以）
2. 展示框为当前准心的样式
3. 设置完毕后按下`Toggle`

### 设置
通过创建 `config.toml` 文件来配置你的设置：
```
.
├── config.toml
├── crosshair.png
├── PadReticle.exe
└── Uninstall PadReticle.lnk
```
目前仅有两个可选项，默认开镜为左肩键，轮询间隔为80ms

`config.toml` 示例：
```toml
aim_key = 8 
polling_interval = 80
```
调整开镜键，请参见 [aim_key 列表](#aim-key-list)。

---

## 维护
有任何建议都可以提**issue**

---

## 许可证
本项目使用 MIT 许可证。

# aim-key List

```rust
pub const DPAD_UP: usize = 0;
pub const DPAD_DOWN: usize = 1;
pub const DPAD_LEFT: usize = 2;
pub const DPAD_RIGHT: usize = 3;
pub const MENU: usize = 4;
pub const SELECT: usize = 5;
pub const LEFT_STICK: usize = 6;
pub const RIGHT_STICK: usize = 7;
pub const LEFT_BUMPER: usize = 8;
pub const RIGHT_BUMPER: usize = 9;
pub const ACTION_A: usize = 12;
pub const ACTION_B: usize = 13;
pub const ACTION_X: usize = 14;
pub const ACTION_Y: usize = 15;
```

