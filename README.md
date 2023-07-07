# rud - Urban Dictionary CLI 💬

rud is a command-line interface (CLI) tool written in Rust that allows you to quickly look up the meaning of words on Urban Dictionary. It provides a simple and efficient way to retrieve definitions for specific words directly from the command line. 🚀

## Prerequisites 📋

Before using rud, make sure you have Rust and Cargo installed on your system. You can install them by following the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). 🛠️

## Installation ⚙️

To install rud, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/your_username/rud.git
   ```

2. Change into the project directory:

   ```bash
   cd rud
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. The built binary will be located at `target/release/rud`. You can either add this location to your system's PATH or run the binary directly using its full path. 👨‍💻

## Usage 🖥️

To use rud, open a terminal and run the following command:

```bash
rud <WORD>
```

Replace `<WORD>` with the word you want to find the meaning of. The program will retrieve the definition from Urban Dictionary and display it on the command line. 📖

### Example 📝

```bash
rud rust
```

This command will retrieve the meaning of the word "rust" from Urban Dictionary and display it on the command line.

## Options 🎛️

rud supports the following options:

- `-h, --help`: Print help information about the usage of rud.
- `-V, --version`: Print the version information of rud.

## Contributing 🤝

Contributions to rud are welcome! If you find any issues or have suggestions for improvements, please open an [issue](https://github.com/ashishbinu/rud/issues) to report a problem or request a feature.

If you would like to contribute code to the project, please follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch with a descriptive name for your feature or bug fix.
3. Make your changes in the codebase.
4. Commit your changes with clear and concise commit messages.
5. Push your branch to your forked repository.
6. Open a [pull request](https://github.com/ashishbinu/rud/pulls) to submit your changes for review.

Thank you for your interest in contributing to rud! Together, we can make it even better. 🙌

## Acknowledgements 🙏

We would like to express our gratitude to the following individuals and projects for their contributions to rud:

- The [Rust](https://www.rust-lang.org/) community for creating a powerful and efficient programming language. 🦀
- The [clap](https://crates.io/crates/clap) crate for providing a convenient and robust command-line argument parsing library for Rust. 📜
- The [scraper](https://crates.io/crates/scraper) crate for enabling HTML scraping capabilities in Rust. 🕷️
- The [ureq](https://crates.io/crates/ureq) crate for simplifying HTTP requests in Rust. 🌐

Your contributions and efforts are greatly appreciated! 🎉

## License 📃

This project is licensed under the [MIT License](https://github.com/ashishbinu/rud/blob/main/LICENSE). 📜

🌟 Make sure to give it a star if you find it useful! 🌟
