# Autodesk Alias MCP Plugin

这是一个面向 Autodesk Alias 的 MCP（Model Context Protocol）插件原型，目标是把部分 Alias 建模与场景操作能力暴露为可调用工具，便于脚本、自动化流程或大模型系统接入。

## 项目概览

该仓库主要由以下部分组成：

- `McpServePlugin_5`：Alias 插件与 MCP 服务实现，负责注册工具、分发请求并与 Alias 运行时交互。
- `alias_lic`：一个轻量命令行客户端，用于调用插件暴露的工具。
- `base_geometry_lib`、`layered_canvas_gdi`、`openalias_rs`：提供几何、绘图与 Alias 绑定支持。

## 当前能力

当前已实现的工具主要覆盖以下几类：

- 对象创建与编辑：创建点、线、方形曲面、分组；平移、删除、分配图层、设置显示模式。
- 曲线与曲面：查询与修改曲线/曲面控制点、显示 CV、提升阶次、进行简单变换。
- 场景与视图：创建图层、保存场景、截图、获取屏幕信息、转换屏幕坐标、创建构造平面。
- 辅助显示：在界面中显示辅助点、箭头、线条、矩形，以及给截图添加坐标尺。

## 使用方式

### 构建

```bash
cargo build --manifest-path alias_lic/Cargo.toml
cargo build --manifest-path McpServePlugin_5/Cargo.toml
```

### 查看可用工具

```bash
cargo run --manifest-path alias_lic/Cargo.toml -- list
```

### 查看某个工具帮助

```bash
cargo run --manifest-path alias_lic/Cargo.toml -- help object_create_point
```

### 调用示例

```bash
cargo run --manifest-path alias_lic/Cargo.toml -- object_create_point --x 0 --y 0 --z 0
```

## 说明与注意事项

- [target/openAlias_C.lib](target/openAlias_C.lib) 是 openalias_rs 连接的 C++ 库文件。
- 生成后的 `McpServePlugin_5` 依赖较少，调用响应较快，且不会影响 Alias 的实时操作体验。
- 当前功能仍不完备，适合用于参考、学习和二次开发，而不是直接作为完整生产级插件使用。
- MCP 服务默认通过本地地址 `127.0.0.1:9000/mcp` 提供调用入口。

## 相关代码位置

- `McpServePlugin_5/src/funcs/`
- `McpServePlugin_5/src/functions.rs`
- `McpServePlugin_5/src/functionsinfo.rs`
