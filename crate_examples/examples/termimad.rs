use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    let result = markdown_to_html("报错信息反馈说明 'qdrant-client' crate 的构建过程中，找不到 `protoc` 安装项。`protoc` 是 Protocol Buffers（protobuf）的编译器。

要解决这个错误，你需要安装 `protoc`，你可以使用 Homebrew（如果你在 macOS 上），或者直接从 protobuf 的 GitHub 发布页下载。

安装 `protoc` 的步骤如下：

1. 如果没有安装 Homebrew，请首先[安装 Homebrew](https://brew.sh/)。如果已经安装，可以直接跳过这一步。

2. 使用 Homebrew 安装 `protobuf`：

```bash
brew install protobuf
```

安装完成后，你的系统就应该已经正确安装了 `protoc`。

如果你还是遇到这个问题，可能是你的 PATH 配置有误，或者安装的 `protoc` 未被正确链接。你可以通过以下命令查看 `protoc` 是否已经在 PATH 中：

```bash
which protoc
```

如果这个命令返回 `protoc` 的路径，说明它已经存在于 PATH 中了。如果没有返回结果，你可能需要手动将其添加至 PATH，或者设置 `PROTOC` 环境变量，以指定你安装的 `protoc` 的路径。

设置环境变量的方法如下：

```bash
export PROTOC=/path/to/your/protoc
```

注意：将 `/path/to/your/protoc` 替换为你的 `protoc` 安装路径。

最后，尝试重新构建你的 Rust 项目应该就不会遇到同样的错误了。", &ComrakOptions::default());

    termimad::print_inline(&result);
    termimad::print_inline("**some** *nested **style*** and `some(code)`");
}
