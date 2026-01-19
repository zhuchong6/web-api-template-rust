# Web API Template Rust

这个项目是一个web开发的模版，用于后续的web开发

## 引入axum框架

在cargo.toml中添加以下依赖：

```toml
axum = "0.8.8"
tokio = { version = "1.49.0", features = ["full"] }
```

在编写handler的时候，可以在handler上面加入`#[debug_handler]`来校验你的handler是不是符合要求，输出具体的错误信息。这个只在debug模式下生效，release模式编译器会自动去掉。
