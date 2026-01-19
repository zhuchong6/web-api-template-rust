# Web API Template Rust

这个项目是一个web开发的模版，用于后续的web开发

## 1.集成axum框架

在cargo.toml中添加以下依赖：

```toml
axum = "0.8.8"
tokio = { version = "1.49.0", features = ["full"] }
```

在编写handler的时候，可以在handler上面加入`#[debug_handler]`来校验你的handler是不是符合要求，输出具体的错误信息。这个只在debug模式下生效，release模式编译器会自动去掉。】

## 2.集成日志框架tracing

在cargo.toml中添加以下依赖：

```toml
tracing = "0.1.44"
tracing-subscriber = "0.3.22"
```

## 3.集成配置功能

在cargo.tol中添加以下依赖：

```toml
### 读取配置
config = { version = "0.15.19", features = ["yaml"] }
### 序列化🍚序列化
serde = { version = "1.0.228", features = ["derive"] }
### 错误处理
anyhow = { version = "1.0.100" }
```
