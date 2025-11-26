import json
import os
import random
import re
import shutil
import sys
import argparse # Import argparse

from rich import print as rprint
from rich.panel import Panel

sys.path.append(os.path.realpath("."))

import inquirer


def show_summary(dir_name, lang, aoc_name):
    rprint(
        Panel.fit(
            f"[bold cyan]🎉 You can now start coding! 🎉[/]\n\n"
            f"[bold white]📁 Directory:[/] [cyan]{dir_name}[/]\n"
            f"[bold white]🧠 Language:[/] [magenta]{lang}[/]\n"
            f"[bold white]👤 AoC Username:[/] [green]{aoc_name}[/]\n",
            title="[bold green]Success[/]",
            border_style="bright_green",
        )
    )


def load_coders():
    if not os.path.exists("coders"):
        return []
    return os.listdir("coders")


def validate_directory_name(current):
    # Check for invalid characters
    directories = load_coders()
    if not re.match(r"^[a-zA-Z0-9_-]+$", current):
        raise inquirer.errors.ValidationError(
            "",
            reason="Directory name contains invalid characters. Use only (a-z, A-Z, 0-9, _, -).",
        )

    if current in directories:
        raise inquirer.errors.ValidationError(
            "", reason=f"'{current}' is already taken or invalid directory name."
        )

    return True


def copy_template(lang, dest_dir):
    lang_lower = lang.lower().replace("+", "p").replace("#", "sharp")
    template_dir = f"scripts/templates/{lang_lower}"
    if not os.path.exists(template_dir):
        rprint(
            f"[yellow]No template found for {lang}, creating empty directory.[/yellow]"
        )
        return

    shutil.copytree(template_dir, dest_dir, dirs_exist_ok=True)


languages_predefined = [ # Make this global
    "Python",
    "JavaScript",
    "C++",
    "Java",
    "Ruby",
    "Go",
    "Rust",
    "TypeScript",
    "Kotlin",
    "PHP",
    "Lua",
    "R",
]

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Create a new Advent of Code coder directory.")
    parser.add_argument("--dir_name", type=str, help="What should your directory be called")
    parser.add_argument("--aoc_name", type=str, help="What is your Advent of Code username")
    parser.add_argument(
        "--lang",
        type=str,
        choices=languages_predefined + ["Random", "Other"],
        help="What language will you be coding in?",
    )
    parser.add_argument(
        "--custom_lang",
        type=str,
        help="Please specify your programming language if 'Other' is selected for --lang",
    )
    parser.add_argument(
        "--confirm",
        action="store_true",
        help="Confirm creating the coder directory with these settings (non-interactive)",
    )

    args = parser.parse_args()

    if args.dir_name and args.aoc_name and args.lang:
        # Non-interactive mode
        dir_name = args.dir_name
        aoc_name = args.aoc_name
        lang = args.lang
        confirm = args.confirm

        # Perform validation similar to inquirer, but for non-interactive mode
        try:
            validate_directory_name(dir_name)
        except inquirer.errors.ValidationError as e:
            rprint(f"[red]Error: {e.reason}[/red]")
            sys.exit(1)
            
        if lang == "Other" and not args.custom_lang:
            rprint("[red]Error: --custom_lang must be specified if --lang is 'Other'.[/red]")
            sys.exit(1)
        elif lang == "Other":
            lang = args.custom_lang

        if not confirm:
            rprint("[red]Aborted creating new coder directory (non-interactive mode requires --confirm).[/red]")
            sys.exit(0)

    else:
        # Interactive mode
        questions = [
            inquirer.Text(
                "dir_name",
                message="What should your directory be called",
                validate=lambda _, x: validate_directory_name(x), # Adapt for inquirer validation
            ),
            inquirer.Text("aoc_name", message="What is your Advent of Code username"),
            inquirer.List(
                "lang",
                message="What language will you be coding in?",
                choices=languages_predefined + ["Random", "Other"],
            ),
            inquirer.Text(
                "custom_lang",
                message="Please specify your programming language",
                ignore=lambda answers: answers["lang"] != "Other",
            ),
            inquirer.Confirm(
                "confirm",
                message="Create the coder directory with these settings?",
                default=True,
            ),
        ]

        answers = inquirer.prompt(questions)
        dir_name = answers["dir_name"]
        aoc_name = answers["aoc_name"]
        lang = answers["lang"]
        confirm = answers["confirm"]

        if confirm is None or not confirm:
            rprint("[red]Aborted creating new coder directory.[/red]")
            sys.exit(0)

        if lang == "Other":
            lang = answers["custom_lang"]


    if lang == "Random":
        # Don't choose a single language here, it will be done per day.
        pass
    
    os.makedirs(f"coders/{dir_name}", exist_ok=True)
    config = {"language": lang, "aoc_username": aoc_name}

    if lang != "None":
        with open(f"coders/{dir_name}/config.json", "w") as f:
            json.dump(config, f, indent=4)
        rprint("[yellow]Created config file[/yellow]")

    for i in range(1, 13):
        day_dir = f"coders/{dir_name}/day_{i:02d}"
        os.makedirs(day_dir, exist_ok=True)

        # Create input.txt
        with open(f"{day_dir}/input.txt", "w") as f:
            pass

        day_lang = lang
        if lang == "Random":
            if 'random_languages_shuffled' not in locals(): # Shuffle only once
                global random_languages_shuffled
                random_languages_shuffled = random.sample(languages_predefined, len(languages_predefined))
            
            # Use modulo to cycle through languages if there are more days than languages
            day_lang = random_languages_shuffled[(i - 1) % len(random_languages_shuffled)]
            
            day_config = {"language": day_lang}
            with open(f"{day_dir}/config.json", "w") as f:
                json.dump(day_config, f, indent=4)

        # Copy template
        if day_lang in languages_predefined:
            copy_template(day_lang, day_dir)
            makefile_template = ""
            if day_lang == "C++":
                makefile_template = "scripts/templates/cpp/Makefile"
            else:
                lang_lower = day_lang.lower().replace("+", "p").replace("#", "sharp")
                makefile_template = f"scripts/templates/makefiles/Makefile.{lang_lower}"
            
            if os.path.exists(makefile_template):
                shutil.copy(makefile_template, f"{day_dir}/Makefile")
                rprint(f"[yellow]Copied Makefile for day {i:02d} ({day_lang})[/yellow]")

    if lang != "None" and lang != "Other":
        rprint(f"[yellow]Created 12 day directories with templates for {lang}[/yellow]")
    else:
        rprint(f"[yellow]Created 12 day directories for {dir_name}[/yellow]")
        rprint(
            "[bold red]Warning:[/] You have chosen not to use a predefined language. The 'make analyze' command will not work unless you manually create the correct file structure and output format."
        )

    show_summary(dir_name, lang, aoc_name)

