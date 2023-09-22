# Posify

Posify is a command-line tool written in Rust that converts negative images to positive ones. It's a simple and fast utility that can be used on Linux, macOS, and Windows systems.

## Features

- Fast image processing
- Supports various image formats (JPEG, PNG, etc.)
- Easy to use

## Usage

To convert a negative image to a positive one, run the following command:

```bash
posify <input_image_path> <output_image_path>
```

- `<input_image_path>`: The path to the negative image you want to convert.
- `<output_image_path>`: The path where you want to save the converted positive image.

## Installation

### Linux

#### From Source

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/posify.git
    ```

2. Navigate to the project directory:

    ```bash
    cd posify
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

4. Move the binary to `/usr/local/bin`:

    ```bash
    sudo mv target/release/posify /usr/local/bin/
    ```

### macOS

The installation steps for macOS are similar to those for Linux.

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/posify.git
    ```

2. Navigate to the project directory:

    ```bash
    cd posify
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

4. Move the binary to `/usr/local/bin`:

    ```bash
    sudo mv target/release/posify /usr/local/bin/
    ```

### Windows

1. Clone the repository or download the source code.

2. Navigate to the project directory and open a Command Prompt or PowerShell window.

3. Build the project:

    ```bash
    cargo build --release
    ```

4. The executable will be in the `target\release` directory. You can move it to a directory that is in your system's PATH or add the `target\release` directory to your PATH.

## Contributing

Feel free to open issues or pull requests if you want to improve the tool.

## License

This project is licensed under the MIT License.