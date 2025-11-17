# Declaring a Module

In Rust, modules are declared using the `mod` keyword to organize code and manage visibility.

## Inline Module

A module can be defined directly within the same file using the `mod` keyword followed by the module name and curly braces containing its contents.

```Rust
mod my_module {
    pub fn my_function() {
        println!("Hello from my_module!");
    }
}

fn main() {
    my_module::my_function();
}
```

## External File Module

A module can be defined in a separate file and declared in the parent module (often `main.rs` or `lib.rs`) using the `mod` keyword followed by the module name and a semicolon. The content of the module resides in a file with the same name (e.g., `my_module.rs`).

```Rust
// main.rs
mod my_module; // Declares the module defined in my_module.rs

fn main() {
    my_module::my_function();
}

// my_module.rs
pub fn my_function() {
    println!("Hello from my_module!");
}
```

## Directory Module

For larger modules, the content can be placed in a directory with the same name as the module, containing a `mod.rs` file. The module is then declared in the parent file using `mod` followed by the directory name.

```Rust
// main.rs
mod my_module; // Declares the module defined in my_module/mod.rs

fn main() {
    my_module::my_function();
}

// my_module/mod.rs
pub fn my_function() {
    println!("Hello from my_module!");
}
```

Visibility and Paths:

* Items within a module are private by default. Use the `pub` keyword to make them publicly accessible.

* Modules form a tree structure, and items within them are accessed using paths (e.g., `module_name::sub_module::item_name`).

* The `use` keyword can be used to bring items or modules into scope for easier access, avoiding repetitive path specifications.
