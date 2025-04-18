# webp-image

A simple Rust tool to quickly save images from the clipboard or from a file path as WebP format.

---

一个用于将剪贴板中的图片或图片文件路径快速保存为 WebP 格式的小工具，基于 Rust 开发。

## Features / 功能简介
- 支持从剪贴板读取图片内容或图片文件路径。
- 自动将图片编码为 WebP 格式并保存到本地。
- 文件路径模式下，WebP 文件与原图同目录同名。
- 剪贴板图片内容模式下，自动生成唯一文件名。

## Installation / 安装

1. . 克隆本仓库并编译：
   ```sh
   git clone <本仓库地址>
   cd webp-image
   cargo build --release
   ```
2. 可执行文件位于 `target/release/webp-image`

## Usage / 使用方法

1. 复制一张图片或图片文件路径到剪贴板。
2. 运行本程序：
   ```sh
   ./target/release/webp-image
   ```
3. 程序会自动检测剪贴板内容并保存 WebP 文件。

### 示例 / Example
- 复制一张图片到剪贴板，运行程序后会在当前目录生成 webp 文件。
- 复制图片文件路径（如 `/Users/xxx/Pictures/1.png`），运行程序后会在同目录生成 `1.webp`。

## Dependencies / 依赖
- [arboard](https://crates.io/crates/arboard)：跨平台剪贴板访问
- [image](https://crates.io/crates/image)：图片处理
- [webp](https://crates.io/crates/webp)：WebP 编码
- [uuid](https://crates.io/crates/uuid)：生成唯一文件名

## Supported Platforms / 支持平台
- macOS、Windows、Linux 主流桌面系统

## Notes / 备注
- 需确保剪贴板内容为图片或图片文件路径。
- 仅支持主流桌面系统。

## Contributing / 贡献
欢迎提交 issue 和 PR 改进本项目。

## License / 许可证
MIT License.