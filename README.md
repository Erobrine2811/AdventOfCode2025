# 🎄 Advent of Code 2025 - A Festive Coding Collaboration! 🎄

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
make analyze DAYS="1-3" CODERS="johnthegoat,hacker" PART=1
```
This command will analyze Part 1 solutions for the `python` and `go` coders for Days 1, 2, and 3.

---

## 🌟 Contributing

We enthusiastically welcome contributions from everyone! Whether you're fixing a bug, adding a new feature, or improving documentation, your help is invaluable. By contributing, you'll not only help make this project better but also learn from a diverse community of coders.

### How You Can Contribute

We follow a standard Forking Workflow to manage contributions and ensure code quality. Please follow these steps:

1.  **Fork the Repository:** Start by forking this repository to your GitHub account. This creates your personal copy where you can make changes.
2.  **Clone Your Fork:** Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/YOUR_GITHUB_USERNAME/AdventOfCode2025.git
    cd AdventOfCode2025
    ```
3.  **Create a New Branch:** Always create a new branch for completing a day or days:
    ```bash
    git checkout -b feature/your-feature-name
    ```
4.  **Make Your Changes:** Implement your solution, fix bugs, or add new features in your new branch. Remember to adhere to the project's conventions.
5.  **Commit Your Changes:** Commit your changes with clear and descriptive commit messages:
    ```bash
    git add .
    git commit -m "feat: Add solution for Day X Part Y"
    ```
6.  **Push to Your Fork:** Push your local branch to your forked repository on GitHub:
    ```bash
    git push origin feature/your-feature-name
    ```
7.  **Create a Pull Request (PR):** Go to the original repository on GitHub, and you'll see an option to create a new Pull Request from your pushed branch.
    *   Ensure your PR targets the `main` branch of this repository.
    *   Provide a clear title and description for your PR, explaining the changes you've made.
    *   Your PR will be reviewed, and once approved, it will be merged into `main`.

Please ensure your changes are thoroughly tested locally before submitting a pull request.

2.  **Respect Others' Code:** It is **critically important** that you **do not modify other contributors' solutions or files outside of your own coder directory (`coders/<your_name>/`).** If you find an issue or have an improvement for someone else's code, please communicate with them directly or open an issue to discuss it.
3.  **Running Analysis on Your Branch:**
    *   To run `make analyze` effectively on your feature branch, you may need to ensure it's up-to-date with the `main` branch. You can do this by running `git pull origin main` on your branch (and resolving any conflicts) before running `make analyze).

### What You Can Contribute

Here are some specific ways you can help:

*   **Add New Language Support:** Want to see your favorite language here? Add a new template, a `Makefile` for the analyzer, and updates to `scripts/new_coder.py` and `scripts/analyze.py`.
*   **Improve Existing Templates/Solutions:** Found a more efficient algorithm? Have a cleaner code style? Share your improvements!
*   **Enhance Analysis Tools:** Help us make `analyze.py` even better by suggesting or implementing new metrics, visualizations, or features.
*   **Bug Fixes:** Found a bug? We appreciate your help in squashing them!
*   **Documentation:** Clear and comprehensive documentation makes everyone's life easier.
*   **Refactoring:** Help improve the codebase's readability and maintainability.

---

## ❄️ Happy Coding, and May Your Solutions Be Merry and Bright! ❄️

> Created by gemini :P
