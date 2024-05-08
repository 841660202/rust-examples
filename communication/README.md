## 模块树

在 Rust 中，创建多级嵌套模块的结构需要在文件系统中适当地组织文件和目录，并在代码中声明模块的层次结构。下面是一个例子，演示了如何创建一个多级嵌套的模块结构。

假设我们有一个名为 `communication` 的顶级模块，它包含两个子模块：`voice` 和 `text`。`voice` 模块进一步包含一个名为 `calls` 的子模块，而 `text` 模块包含一个名为 `messages` 的子模块。

文件结构如下：

```
src/
|-- communication/
|   |-- mod.rs
|   |-- voice/
|   |   |-- mod.rs
|   |   `-- calls.rs
|   `-- text/
|       |-- mod.rs
|       `-- messages.rs
`-- main.rs
```

在这个结构中：

- `communication` 是一个目录，其中 `mod.rs` 声明子模块 `voice` 和 `text`。
- `voice` 和 `text` 都是目录，每个目录都有自己的 `mod.rs` 来声明它们的子模块。
- `calls` 和 `messages` 是文件，包含相应子模块的实现。

现在，让我们填充这些文件以创建模块和函数：

**src/communication/mod.rs**

```rust
pub mod voice;
pub mod text;
```

**src/communication/voice/mod.rs**

```rust
pub mod calls;

pub fn speak() {
    println!("Speaking...");
}
```

**src/communication/voice/calls.rs**

```rust
pub fn make_call() {
    println!("Making a call...");
}
```

**src/communication/text/mod.rs**

```rust
pub mod messages;

pub fn write() {
    println!("Writing a text...");
}
```

**src/communication/text/messages.rs**

```rust
pub fn send_message() {
    println!("Sending a message...");
}
```

最后，在 `main.rs` 文件中，我们可以使用这些模块和函数：

**src/main.rs**

```rust
mod communication;

fn main() {
    communication::voice::speak();
    communication::voice::calls::make_call();
    communication::text::write();
    communication::text::messages::send_message();
}
```

在这个例子中，我们可以看到如何在 Rust 中创建和使用多级嵌套模块。每个模块都有自己的目录和 `mod.rs` 文件（或者是一个与模块同名的 `.rs` 文件），这些文件声明了模块的内容并提供了模块的实现。通过这种方式，我们可以构建出清晰和组织良好的模块结构。
