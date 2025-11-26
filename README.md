# 🎄 Advent of Code 2025 - A Festive Coding Collaboration! 🎄

![Festive Banner](https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExaGNvMXZkN3R0c2NpbjB3d2ZtaWJ0bjVwZHM1Mm1veXp2aHVpZHU5eiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/26tOZlKO9QxXcH0gE/giphy.gif)

## ✨ Greetings, Fellow Coder, and Welcome to the Festivities! ✨

We're so excited you're joining us for this year's Advent of Code! This repository is a collaborative space for us to share our solutions, learn from each other, and celebrate the joy of coding during this festive season. Whether you're a seasoned pro or just starting your coding journey, we're thrilled to have you.

Let's make this a December filled with brilliant code, shared discoveries, and plenty of holiday cheer! 🎅🦌🎁

---

## 🚀 Getting Started

### 0. Setting up the Python Virtual Environment

This project uses a Python virtual environment to manage dependencies. Follow these steps to set it up:

1.  **Create the venv:**
    ```bash
    python3 -m venv venv
    ```
2.  **Activate the venv:**
    ```bash
    source venv/bin/activate
    ```
3.  **Install dependencies:**
    ```bash
    pip install -r requirements.txt
    ```
Remember to activate your virtual environment in each new terminal session before running any Python-related commands for this project.

### 1. Creating Your Coding Environment

To get started, you'll need to create your own coder environment. This will set you up with a personal directory containing templates for all 25 days of Advent of Code.

Run the following command:

```bash
make new
```

This will launch an interactive setup guide that will ask for:
*   **Your Coder Name:** A unique name for your directory.
*   **Your Advent of Code Username:** To keep track of who's who.
*   **Your Language of Choice:** Pick from our list of supported languages!

You can also run `make new` non-interactively with arguments (useful for scripting or testing):

```bash
# After activating your venv, the 'python' command will point to the venv's Python.
python scripts/new_coder.py --dir_name my_coder --aoc_name my_username --lang Python --confirm
```

### 2. Supported Languages

We currently have templates and analysis support for a wide range of languages:

*   Python
*   JavaScript
*   C++
*   Java
   *   Ruby
*   Go
*   Rust
*   TypeScript
*   Kotlin
*   PHP
*   Lua
*   R
*   **Random:** Each day will be assigned a unique language from the supported list!

If you don't see your favorite language, feel free to contribute a template!

---

## 💻 How to Use This Repository

### Running Your Solution

Once you've created your coder environment, you can easily run your solution for any given day. You will find a `Makefile` within each day's directory (`coders/<your_name>/day_XX/`). Navigate into the specific day's directory and use `make`:

```bash
# Example: Running solution for day 1 of your coder
cd coders/my_coder/day_01

# Run both parts for Day 1
make

# Run just Part 1 for Day 1
make part1
```

### Analyzing All Solutions

To see how your solution stacks up against others, you can use our analysis script. This will run the solutions for all coders and display a performance comparison.

From the root of the repository, run:

```bash
make analyze DAY=<day>
```

**Available Options for `make analyze`:**

*   `DAY=<day>`: Specify a single day to analyze (1-25).
*   `CODERS="<coder1>,<coder2>"`: Filter by one or more coder names.
*   `PART=<1 or 2>`: Analyze only a specific part of the solution.
*   `DAYS="<day1,day2-dayX>"`: Specify a list or range of days to analyze.

**Example:**

```bash
make analyze DAYS="1-3" CODERS="python,go" PART=1
```
This command will analyze Part 1 solutions for the `python` and `go` coders for Days 1, 2, and 3.

---

## 🌟 Contributing

This is a collaborative project, and we welcome contributions! Here's how you can help:

*   **Add a New Language:** If you'd like to add support for a new language, please open a pull request with a template, a `Makefile` for the analyzer, and the necessary updates to `scripts/new_coder.py` and `scripts/analyze.py`.
*   **Improve a Template:** Have a better way to structure a solution? We'd love to see it!
*   **Enhance the Analysis:** Feel free to improve the `analyze.py` script to provide even more insightful metrics.

## ❄️ Happy Coding, and May Your Solutions Be Merry and Bright! ❄️

> Created by gemini
