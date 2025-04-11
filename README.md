# UserActivityTrainer

An application built with Rust and Slint to capture user mouse movements and keystrokes, intended for training a machine learning model to perform specific tasks.

## About

This project uses the `device_query` crate to listen for global mouse and keyboard events and records this activity. The collected data can then be used to train a machine learning model. The user interface is built using the [Slint](https://slint.rs/) toolkit.

**Note:** The specific task the ML model is trained for is currently under development/not specified.

![image](https://github.com/user-attachments/assets/ce155969-c3ee-440d-9bd3-b9376ca0b7c3)


## Features

- Captures mouse movement coordinates.
- Captures keyboard key presses.
- Provides a basic UI using Slint.
- (Potentially) Includes functionality to save/load captured data.
- (Potentially) Includes ML model training and inference logic.

## Usage

### Prerequisites

1.  **Install Rust:** Follow the [getting-started guide](https://www.rust-lang.org/learn/get-started). You'll need `rustc` and `cargo`.
2.  **Permissions for Input Monitoring:** This application captures global keyboard and mouse input using the `device_query` crate. Depending on your operating system, you may need to grant special permissions:
    - **macOS:** You might need to grant Accessibility access.
    - **Linux (X11):** Usually works out of the box.
    - **Linux (Wayland):** Input monitoring might be restricted.
    - **Windows:** May require running the application as an administrator.
      Consult the `device_query` documentation or your OS guidelines for specific requirements.

### Building and Running

1.  Clone or download this repository.
2.  Navigate to the project directory:
    ```bash
    cd UserActivityTrainer # Or your chosen project directory name
    ```
3.  Build the application:
    ```bash
    cargo build --release
    ```
4.  Run the application:
    ```bash
    cargo run --release
    ```
    Alternatively, run the compiled binary directly from `./target/release/`.

## Development

We recommend using an IDE with Rust support (like VS Code with the rust-analyzer extension). For the Slint UI parts (`.slint` files), consider using the [Slint LSP integration](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md) or the [Slint VS Code extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

- Define the specific task for the ML model.
- Implement data saving/loading mechanisms.
- Integrate the ML model training and usage logic.
- Refine the Slint UI.

## License

(Please specify your chosen license here, e.g., This project is licensed under the MIT License - see the LICENSE file for details.)
