# Install
https://www.rust-lang.org/tools/install


- ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
- ```source ~/.bashrc```

Check installation

- ```rustc --version```

# VS code extension
https://code.visualstudio.com/docs/languages/rust

- rust-analyzer

# Package creation
https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html

For creating a new package with a bin output and for initializing a new git repository

- ```cargo new hello_world --bin```

For creating a new package library and for initializing a new git repository

- ```cargo new hello_world --lib```

To avoid initializing a git repo pass ```--vcs none```

# Build

- ```cargo build``` -> without optimization
- ```cargo run``` -> build and run without optimization
- ```cargo build --release``` -> build with optimization

# Build crates documentation

- ```cargo doc --open```

