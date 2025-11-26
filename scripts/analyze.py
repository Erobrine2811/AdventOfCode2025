import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

from rich.console import Console
from rich.live import Live
from rich.markup import escape
from rich.progress import track
from rich.table import Table



# --- Data Classes ---

@dataclass
class RunResult:
    """Represents the result of a single run (part 1 or 2)."""
    success: bool
    runtime: float = 0.0
    output: str = ""
    error: str = ""


@dataclass
class CoderDayResult:
    """Represents the analysis result for a single coder on a given day."""
    coder_name: str
    language: str = "[red]N/A[/red]"
    part1: Optional[RunResult] = None
    part2: Optional[RunResult] = None
    status: str = "[blue]Pending[/blue]"

    @property
    def total_runtime(self) -> float:
        total = 0.0
        if self.part1 and self.part1.success:
            total += self.part1.runtime
        if self.part2 and self.part2.success:
            total += self.part2.runtime
        return total

# --- Core Logic ---

def find_executable(name: str) -> bool:
    """Check if an executable exists in PATH."""
    return shutil.which(name) is not None

def get_coders() -> List[str]:
    """Returns a list of all coder directories."""
    if not os.path.exists("coders"):
        return []
    return [d for d in os.listdir("coders") if os.path.isdir(os.path.join("coders", d))]


def get_coder_config(coder_dir: str) -> Optional[Dict]:
    """Reads the config.json for a coder."""
    config_path = os.path.join("coders", coder_dir, "config.json")
    if not os.path.exists(config_path):
        return None
    with open(config_path, "r") as f:
        return json.load(f)


def run_command(command: str, cwd: str, timeout: int = 30) -> Tuple[bool, str, str]:
    """Runs a shell command and captures output."""
    try:
        process = subprocess.run(
            command,
            shell=True,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=cwd,
        )
        return process.returncode == 0, process.stdout, process.stderr
    except subprocess.TimeoutExpired:
        return False, "", "Command timed out."
    except Exception as e:
        return False, "", f"An unexpected error occurred: {e}"


def get_day_language(coder: str, day: int) -> Optional[str]:
    """Gets the language for a specific day, falling back to the coder's main language."""
    day_config_path = os.path.join("coders", coder, f"day_{day:02d}", "config.json")
    if os.path.exists(day_config_path):
        with open(day_config_path, "r") as f:
            config = json.load(f)
            return config.get("language")

    coder_config = get_coder_config(coder)
    if coder_config:
        return coder_config.get("language")

    return None

def execute_part(language: str, day_path: str, part: str) -> RunResult:
    """Executes a single part of a day's solution using make."""
    make_vars = ""
    if language == "Python":
        make_vars = f"PYTHON={sys.executable}"
    
    command = f"make {make_vars} {part}"

    start_time = time.time()
    success, stdout, stderr = run_command(command, cwd=day_path)
    end_time = time.time()
    
    if not success:
        return RunResult(success=False, error=stderr or stdout)

    # Simplified time parsing from solution output
    runtime_match = re.search(r"Completed in ([\d.]+) seconds", stdout)
    runtime = float(runtime_match.group(1)) if runtime_match else (end_time - start_time)

    return RunResult(success=True, runtime=runtime, output=stdout)


def analyze_coder_day(coder: str, day: int, part_filter: Optional[int]) -> CoderDayResult:
    """Analyzes a single coder's solutions for a given day."""
    result = CoderDayResult(coder_name=coder)
    
    language = get_day_language(coder, day)
    if not language:
        result.status = "[red]Language not found[/red]"
        return result
    result.language = language
    
    day_path = os.path.join("coders", coder, f"day_{day:02d}")

    if not os.path.exists(day_path):
        result.status = f"[yellow]Day {day:02d} not found[/yellow]"
        return result

    # --- Part Execution ---
    parts_to_run = [1, 2] if not part_filter else [part_filter]
    for part_num in parts_to_run:
        result.status = f"[yellow]Running Part {part_num}...[/yellow]"
        part_result = execute_part(result.language, day_path, f"part{part_num}")
        if part_num == 1:
            result.part1 = part_result
        else:
            result.part2 = part_result
        
        if not part_result.success:
            result.status = f"[red]Part {part_num} Error[/red]\n[dim]{escape(part_result.error)}[/dim]"
            break  # Stop if a part fails
    else: # Runs if the loop completes without break
        result.status = "[green]Success[/green]"

    return result

# --- UI and Main ---

def create_results_table(day: int, part_filter: Optional[int]) -> Table:
    """Creates the Rich table for displaying results."""
    table = Table(title=f"Advent of Code - Day {day:02d} Analysis")
    table.add_column("Coder", justify="left", style="cyan")
    table.add_column("Language", justify="left", style="magenta")
    if part_filter in [1, None]:
        table.add_column("Part 1 Runtime (s)", justify="right", style="green")
    if part_filter in [2, None]:
        table.add_column("Part 2 Runtime (s)", justify="right", style="green")
    table.add_column("Total Runtime (s)", justify="right", style="yellow")
    table.add_column("Status", justify="left", style="white")
    return table

def add_result_to_table(table: Table, result: CoderDayResult, part_filter: Optional[int]):
    """Adds a row to the results table."""
    def format_runtime(part_result: Optional[RunResult]):
        if not part_result: return "[dim]N/A[/dim]"
        return f"{part_result.runtime:.4f}" if part_result.success else "[red]Failed[/red]"

    row = [result.coder_name, result.language]
    if part_filter in [1, None]:
        row.append(format_runtime(result.part1))
    if part_filter in [2, None]:
        row.append(format_runtime(result.part2))
    
    total_runtime_str = f"{result.total_runtime:.4f}" if result.total_runtime >= 0 else "[dim]N/A[/dim]"
    row.extend([total_runtime_str, result.status])
    table.add_row(*row)

def parse_days_arg(days_arg: str) -> List[int]:
    """Parses the --days argument string (e.g., '1,3-5,7') into a list of integers."""
    days = set()
    if not days_arg:
        return []
    for part in days_arg.split(","):
        if "-" in part:
            start, end = map(int, part.split("-"))
            days.update(range(start, end + 1))
        else:
            days.add(int(part))
    return sorted(list(days))

def main():
    parser = argparse.ArgumentParser(description="Analyze Advent of Code solutions.")
    parser.add_argument("--day", type=int, help="The day to analyze (1-25).")
    parser.add_argument("--days", type=str, help="A comma-separated list of days or ranges to analyze (e.g., '1,3-5,7').")
    parser.add_argument("--coders", type=str, help="Comma-separated list of coder names to analyze.")
    parser.add_argument("--part", type=int, choices=[1, 2], help="Specify which part to analyze (1 or 2).")
    args = parser.parse_args()

    console = Console()

    days_to_run = []
    if args.days:
        days_to_run = parse_days_arg(args.days)
    elif args.day:
        days_to_run = [args.day]

    if not days_to_run:
        console.print("[red]Error: You must specify which day(s) to analyze using --day or --days.[/red]")
        sys.exit(1)

    all_coders = get_coders()
    coders_to_run = [c.strip() for c in args.coders.split(",")] if args.coders else all_coders

    if not coders_to_run:
        console.print("[yellow]No coders found to analyze.[/yellow]")
        return
        
    for day in days_to_run:
        if not 1 <= day <= 25:
            console.print(f"[red]Error: Day {day} is invalid. Must be between 1 and 25.[/red]")
            continue
            
        results = {coder: CoderDayResult(coder_name=coder) for coder in coders_to_run}
        
        table = create_results_table(day, args.part)
        for coder in coders_to_run:
            add_result_to_table(table, results[coder], args.part)

        with Live(table, console=console, screen=False, refresh_per_second=4, vertical_overflow="visible") as live:
            for coder in track(coders_to_run, description=f"Analyzing Day {day:02d}..."):
                result = analyze_coder_day(coder, day, args.part)
                results[coder] = result
                
                new_table = create_results_table(day, args.part)
                for res_coder in coders_to_run:
                    add_result_to_table(new_table, results[res_coder], args.part)
                live.update(new_table)


if __name__ == "__main__":
    main()