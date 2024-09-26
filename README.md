## Luap

Luap is a simple Lua Package Manager. It is designed to be simple and easy to use. Luap is based on GitHub for package management. With Luap, you can easily download, install, and manage Lua packages from GitHub. Whether for personal projects or team collaboration, Luap helps you efficiently manage dependencies, ensuring consistency and maintainability of your projects. It supports cloning packages from GitHub repositories and automatically handles submodule initialization, making package management more convenient.

## Installation

To install Luap, you can either download the precompiled binaries from the [releases](https://github.com/CppCXY/luap/releases) page or compile it yourself.

### Download Precompiled Binaries

1. Go to the [releases](https://github.com/your-repo/luap/releases) page.
2. Download the appropriate binary for your operating system.
3. Add the `luap` binary to your PATH.

### Compile from Source

Clone the repository:
   ```bash
   git clone https://github.com/cppCXY/luap.git
   cd luap
   cargo build --release -p luap
   ```
If your computer does not have SSL, you may need to add `--features "compile_ssl"`: for example:
   ```bash
   cargo build --release -p luap --features "compile_ssl".
   ```
## Usage
