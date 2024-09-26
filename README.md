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
   cargo build --release -p luap --features "compile_ssl"
   ```
## Usage

### Init

To initialize a new Lua project, run the following command in your project directory:
   ```bash
   luap init
   ```
This will create a `package.toml` file in your project directory. You can edit this file to specify the packages you want to use in your project.

### Add

To add a package to your project, run the following command:
   ```bash
   luap add <package-name> <github-repo>
   ```
For example:
   ```bash
   luap add resty https://github.com/LuaCATS/openresty.git
   ```
This will add the `resty` package from the `LuaCATS/openresty` repository to your project.

### Install

To install the packages specified in your `package.toml` file, run the following command:
   ```bash
   luap install
   ```
This will clone the packages from GitHub and initialize any submodules.

### Update

To update the packages in your project, run the following command:
   ```bash
   luap update
   ```
This will update the packages to the latest version.

To update a specific package, run the following command:
   ```bash
   luap update <package-name>
   ```
For example:
   ```bash
   luap update resty
   ```
This will update the `resty` package to the latest version.

### Remove

To remove a package from your project, run the following command:
   ```bash
   luap remove <package-name>
   ```
For example:
   ```bash
   luap remove resty
   ```
This will remove the `resty` package from your project.

