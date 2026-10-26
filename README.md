# tlu-sh-parser-demo

A minimal, interactive command-line variable parser prototype built in Rust. This project serves as a proof-of-concept and the very first component for a future custom minimalist shell ecosystem called **tlu-sh**.

## Features

- **Interactive REPL Loop:** Runs continuously inside the terminal waiting for user commands.
- **Strict Clean Syntax:** Parses variable assignments based on the conceptual `ex` syntax.
- **Safe Input Processing:** Safely filters malformed text and implements strict error handling without panicking.

## Syntax Example

To define a variable inside the prompt, use the `ex` command followed by the variable name and its value separated by `=`:

```text
~> ex PATH=/usr/bin
Variable: PATH
Value: /usr/bin
```

If the input format is invalid or broken, the parser drops it safely:

```text
~> bad_input
Idiot
```

## How to Run on Linux

Follow these steps to clone, build, and run the parser on your Linux machine.

### Prerequisites

Make sure you have Git and Rust/Cargo installed. If you don't have Rust yet, install it via:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation & Execution

1. Clone this repository:
   ```bash
   git clone https://github.com/o0travaa/tlu-sh-parser-demo
   ```

2. Navigate into the project directory:
   ```bash
   cd tlu-sh-parser-demo
   ```

3. Run the interactive parser using Cargo:
   ```bash
   cargo run
   ```

## Architecture Notes

This prototype utilizes strict one-way input isolation logic and lightweight stack-allocated string tokenization mechanics via native Rust iterators (`split`, `next`, and `trim`). Built with modern software safety standards in mind.
