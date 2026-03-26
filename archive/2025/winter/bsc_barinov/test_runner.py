#!/usr/bin/env python3

import subprocess
import sys
from pathlib import Path
import shutil
import os
import time
import glob
import argparse

# Global options
ignore_stderr = False

def compile_c_to_arm(file: str, arm_binary: str, extra_compiler_args=None):
    """
    Compile a C file to an ARM executable using aarch64-unknown-linux-gnu-gcc.
    """
    file = Path(file)
    if extra_compiler_args is None:
        extra_compiler_args = []
    try:
        if file.suffix == ".c":
            subprocess.run(
                [
                    "aarch64-unknown-linux-gnu-gcc",
                    "-fno-stack-protector",
                    "-fPIE",
                    "-pie",
                    "-mgeneral-regs-only",
                    "-O0",
                    str(file),
                    *extra_compiler_args,
                    "-o",
                    str(arm_binary),
                ],
                capture_output=True,
                check=True
            )
        else:
            with open(file) as f:
                first_line = f.readline()
            link_stdlib = "TESTRUNNER_LINKSTDLIB" in first_line
            cmd = ["aarch64-unknown-linux-gnu-gcc"]
            if not link_stdlib:
                cmd.append("-nostdlib")
            cmd += [str(file), *extra_compiler_args, "-o", str(arm_binary)]
            subprocess.run(
                cmd,
                capture_output=True,
                check=True
            )
        return True, "Compilation successful"
    except subprocess.CalledProcessError as e:
        return False, f"Compilation failed: {e.stderr.decode()}"
    except FileNotFoundError:
        return False, "aarch64-unknown-linux-gnu-gcc not found"

def translate_arm_to_x86(arm_binary, output_basename, sgtranslator_path, config_path):
    """
    Translate an ARM binary to x86 using sgtranslator.
    """
    try:
        subprocess.run(
            [str(sgtranslator_path), str(arm_binary), "-o", str(output_basename), "-c", str(config_path)],
            capture_output=True,
            check=True,
            timeout=30
        )
        return True, "Translation successful"
    except subprocess.CalledProcessError as e:
        return False, f"Translation failed: {e.stderr.decode()}"
    except subprocess.TimeoutExpired:
        return False, "Translation timed out"
    except FileNotFoundError:
        return False, f"sgtranslator not found at {sgtranslator_path}"

def run_arm(binary_path):
    """
    Run an ARM binary using QEMU and capture stdout, stderr, and exit code.
    """
    try:
        result = subprocess.run(
            ["qemu-aarch64", str(binary_path)],
            capture_output=True,
            text=True,
            timeout=100
        )
        return result.stdout, result.stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "Timeout", -1
    except FileNotFoundError:
        return "", "qemu-aarch64 not found", -1

def run_x86(binary_path):
    """
    Run the translated x86 binary and capture stdout, stderr, and exit code.
    """
    try:
        result = subprocess.run(
            [str(binary_path)],
            capture_output=True,
            text=True,
            timeout=10
        )
        return result.stdout, result.stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "Timeout", -1
    except FileNotFoundError:
        return "", "Binary not found", -1

def compare_outputs(arm_out, x86_out, ignore_stderr=False):
    """
    Compare outputs. Returns True if they match.
    If ignore_stderr is True, stderr differences are not considered failures.
    """
    arm_stdout, arm_stderr, arm_code = arm_out
    x86_stdout, x86_stderr, x86_code = x86_out

    if arm_code != x86_code:
        return False, f"Exit codes differ: ARM={arm_code}, x86={x86_code}"
    if arm_stdout != x86_stdout:
        return False, f"Stdout differs:\nARM:\n{repr(arm_stdout)}\nX86:\n{repr(x86_stdout)}"
    if not ignore_stderr and arm_stderr != x86_stderr:
        return False, f"Stderr differs:\nARM:\n{repr(arm_stderr)}\nX86:\n{repr(x86_stderr)}"

    return True, "Outputs match"

def write_test_log(log_dir, success, message, arm_out, x86_out):
    """
    Write test results to separate files in the log directory.
    """
    arm_stdout, arm_stderr, arm_code = arm_out
    x86_stdout, x86_stderr, x86_code = x86_out
    
    # Main test result log
    with open(log_dir / "result.log", "w") as f:
        if success:
            f.write("Test passed successfully.\n\n")
        else:
            f.write(f"Test failed.\n")
            f.write(f"Reason: {message}\n\n")
        
        f.write("=== ARM Execution ===\n")
        f.write(f"Exit code: {arm_code}\n")
        f.write("See arm.stdout and arm.stderr for output.\n\n")
        
        f.write("=== x86 Execution ===\n")
        f.write(f"Exit code: {x86_code}\n")
        f.write("See x86.stdout and x86.stderr for output.\n")
    
    # Separate output files
    with open(log_dir / "arm.stdout", "w") as f:
        f.write(arm_stdout)
    
    with open(log_dir / "arm.stderr", "w") as f:
        f.write(arm_stderr)
    
    with open(log_dir / "x86.stdout", "w") as f:
        f.write(x86_stdout)
    
    with open(log_dir / "x86.stderr", "w") as f:
        f.write(x86_stderr)

def cleanup(files_to_remove):
    """
    Clean up generated files.
    """
    for file_path in files_to_remove:
        try:
            if Path(file_path).exists():
                Path(file_path).unlink()
        except Exception as e:
            print(f"Warning: Could not remove {file_path}: {e}")

def write_cmd_log(log_path, test_file, arm_binary, x86_binary, sgtranslator_path, config_path, extra_compiler_args):
    """
    Write a cmd_log file with the commands used to reproduce the test.
    All paths are converted to absolute paths.
    """
    # Convert all paths to absolute
    test_file = Path(test_file).absolute()
    arm_binary = Path(arm_binary).absolute()
    x86_binary = Path(x86_binary).absolute()
    sgtranslator_path = Path(sgtranslator_path).absolute()
    config_path = Path(config_path).absolute()
    
    is_c = test_file.suffix == ".c"
    
    with open(log_path, "w") as f:
        f.write("# Commands to reproduce this test\n\n")
        
        # Compilation command
        extra_args = " " + " ".join(extra_compiler_args) if extra_compiler_args else ""

        if is_c:
            f.write(f"# Compile C to ARM:\n")
            f.write(f"aarch64-unknown-linux-gnu-gcc -fno-stack-protector {test_file}{extra_args} -o {arm_binary}\n\n")
        else:
            with open(test_file) as src:
                first_line = src.readline()
            nostdlib_flag = "" if "TESTRUNNER_LINKSTDLIB" in first_line else "-nostdlib "
            f.write(f"# Compile assembly to ARM:\n")
            f.write(f"aarch64-unknown-linux-gnu-gcc {nostdlib_flag}{test_file}{extra_args} -o {arm_binary}\n\n")
        
        # Translation command
        f.write(f"# Translate ARM to x86:\n")
        f.write(f"{sgtranslator_path} {arm_binary} -o {arm_binary.stem} -c {config_path}\n\n")
        
        # Run commands
        f.write(f"# Run ARM binary:\n")
        f.write(f"qemu-aarch64 {arm_binary}\n\n")
        
        f.write(f"# Run x86 binary:\n")
        f.write(f"{x86_binary}\n")

def move_core_dumps(test_name, fail_dir, before_time):
    """
    Find and move any core dumps created since before_time to the fail_dir.
    """
    home_dir = Path.home()
    # Look for core files in home directory
    core_patterns = [
        home_dir / "core",
        home_dir / "core.*"
    ]
    
    for pattern in core_patterns:
        if "*" in str(pattern):
            core_files = glob.glob(str(pattern))
        else:
            if pattern.exists():
                core_files = [str(pattern)]
            else:
                core_files = []
        
        for core_file in core_files:
            core_path = Path(core_file)
            if core_path.exists():
                # Check if the file was modified after before_time
                mtime = core_path.stat().st_mtime
                if mtime > before_time:
                    dest = fail_dir / f"{test_name}.core"
                    try:
                        shutil.move(str(core_path), str(dest))
                    except Exception as e:
                        pass  # Silently ignore if we can't move it

def print_progress(passed, failed, current, total):
    """
    Print a progress bar that fills the entire terminal width.
    Format: [P: passed, F: failed] ------->......... [current / total]
    """
    # Get terminal width
    term_width = os.get_terminal_size().columns - 1
    
    # Create prefix and suffix
    prefix = f"[P: {passed}, F: {failed}] "
    suffix = f" [{current}/{total}]"
    
    # Calculate available space for the bar
    available_width = term_width - len(prefix) - len(suffix)
    
    if available_width < 1:
        available_width = 1
    
    # Calculate filled portion based on progress
    progress = current / total if total > 0 else 0
    filled = int(available_width * progress)
    
    # Build the bar
    bar = "-" * filled + ">" + "." * (available_width - filled - 1)
    
    return f"\r{prefix}{bar}{suffix}"

def run_tests(examples_dir, sgtranslator_path, config_path, test_name_filter=None, extra_compiler_args=None, keep_all=False):
    global ignore_stderr
    examples_dir = Path(examples_dir)
    if extra_compiler_args is None:
        extra_compiler_args = []
    all_tests = sorted(examples_dir.glob("*.c")) + sorted(examples_dir.glob("*.s"))
    
    # Filter tests if a specific test is requested
    if test_name_filter:
        tests = [t for t in all_tests if t.stem == test_name_filter]
        if not tests:
            print(f"Test '{test_name_filter}' not found in {examples_dir}")
            print(f"Available tests: {', '.join(t.stem for t in all_tests)}")
            sys.exit(1)
    else:
        tests = all_tests

    if not tests:
        print("No .s/.c files found in the examples directory.")
        sys.exit(1)

    total = len(tests)
    passed = 0
    failed = 0
    failed_tests = []

    print(f"Found {total} test(s) to compile and run.\n")

    # Create directory for test execution
    test_dir = Path("executed_tests")
    # Clean up old test directory if it exists
    if test_dir.exists():
        shutil.rmtree(test_dir)
    test_dir.mkdir()

    for idx, test in enumerate(tests, 1):
        test_name = test.stem
        test_id = f"{test.stem}_{test.suffix[1:]}"  # e.g. "factorial_c" or "factorial_s"
        # Record time before test execution for core dump detection
        test_start_time = time.time()
        
        # Use temp location for files during execution
        test_file = test_dir / test.name
        shutil.copy2(test, test_file)
        
        arm_binary = test_dir / f"{test_id}.arm"
        x86_binary = test_dir / f"{test_id}.x86"
        
        # Step 1: Compile to ARM
        success, message = compile_c_to_arm(test_file, arm_binary, extra_compiler_args)
        if not success:
            failed += 1
            print(print_progress(passed, failed, idx, total), end="", flush=True)
            # Create subdirectory for failed test
            fail_dir = test_dir / test_id
            fail_dir.mkdir(exist_ok=True)
            shutil.move(str(test_file), str(fail_dir / test.name))
            with open(fail_dir / f"{test_id}.log", "w") as f:
                f.write(f"Compilation failed:\n{message}\n")
            write_cmd_log(fail_dir / "cmd_log", 
                         fail_dir / test.name, 
                         fail_dir / f"{test_id}.arm",
                         fail_dir / f"{test_id}.x86",
                         sgtranslator_path, config_path, extra_compiler_args)
            move_core_dumps(test_id, fail_dir, test_start_time)
            failed_tests.append(test_name)
            continue

        # Step 2: Translate ARM to x86
        success, message = translate_arm_to_x86(arm_binary, test_dir / test_id, sgtranslator_path, config_path)
        if not success:
            failed += 1
            print(print_progress(passed, failed, idx, total), end="", flush=True)
            # Create subdirectory for failed test
            fail_dir = test_dir / test_id
            fail_dir.mkdir(exist_ok=True)
            shutil.move(str(test_file), str(fail_dir / test.name))
            if arm_binary.exists():
                shutil.move(str(arm_binary), str(fail_dir / f"{test_id}.arm"))
            with open(fail_dir / f"{test_id}.log", "w") as f:
                f.write(f"Translation failed:\n{message}\n")
            write_cmd_log(fail_dir / "cmd_log",
                         fail_dir / test.name,
                         fail_dir / f"{test_id}.arm",
                         fail_dir / f"{test_id}.x86",
                         sgtranslator_path, config_path, extra_compiler_args)
            move_core_dumps(test_id, fail_dir, test_start_time)
            failed_tests.append(test_name)
            continue

        # Step 3: Run ARM binary
        arm_out = run_arm(arm_binary)

        # Step 4: Run x86 binary
        x86_out = run_x86(x86_binary)

        # Step 5: Compare outputs
        success, message = compare_outputs(arm_out, x86_out, ignore_stderr)

        if success:
            passed += 1
            print(print_progress(passed, failed, idx, total), end="", flush=True)
            if keep_all:
                # Keep successful test in directory
                pass_dir = test_dir / test_id
                pass_dir.mkdir(exist_ok=True)
                shutil.move(str(test_file), str(pass_dir / test.name))
                shutil.move(str(arm_binary), str(pass_dir / f"{test_id}.arm"))
                shutil.move(str(x86_binary), str(pass_dir / f"{test_id}.x86"))
                write_test_log(pass_dir, True, "", arm_out, x86_out)
                write_cmd_log(pass_dir / "cmd_log",
                             pass_dir / test.name,
                             pass_dir / f"{test_id}.arm",
                             pass_dir / f"{test_id}.x86",
                             sgtranslator_path, config_path, extra_compiler_args)
            else:
                # Clean up passing test files
                cleanup([test_file, arm_binary, x86_binary])
        else:
            failed += 1
            print(print_progress(passed, failed, idx, total), end="", flush=True)
            # Create subdirectory for failed test
            fail_dir = test_dir / test_id
            fail_dir.mkdir(exist_ok=True)
            shutil.move(str(test_file), str(fail_dir / test.name))
            shutil.move(str(arm_binary), str(fail_dir / f"{test_id}.arm"))
            shutil.move(str(x86_binary), str(fail_dir / f"{test_id}.x86"))
            write_test_log(fail_dir, False, message, arm_out, x86_out)
            write_cmd_log(fail_dir / "cmd_log",
                         fail_dir / test.name,
                         fail_dir / f"{test_id}.arm",
                         fail_dir / f"{test_id}.x86",
                         sgtranslator_path, config_path, extra_compiler_args)
            move_core_dumps(test_id, fail_dir, test_start_time)
            failed_tests.append(test_name)

    print("\n")  # New line after progress bar
    print(f"Summary: {passed}/{total} tests passed")
    
    if failed_tests:
        print(f"\nFailed tests: {', '.join(failed_tests)}")
        print(f"Test directories are in: {test_dir.absolute()}")
    elif keep_all:
        print(f"\nAll tests passed! Keeping all directories for inspection in: {test_dir.absolute()}")
    else:
        print(f"All tests passed! Cleaning up {test_dir}...")
        shutil.rmtree(test_dir)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="ARM to x86 translator test runner - compiles, translates, and validates executables"
    )
    
    parser.add_argument(
        "-d",
        "--test-dir",
        required=True,
        help="Directory containing .c and .s source files to test"
    )
    parser.add_argument(
        "-t",
        "--sgtranslator-path",
        default=None,
        help="Path to the sgtranslator binary (default: search in PATH)"
    )
    parser.add_argument(
        "-c",
        "--config-path",
        default="./config.json",
        help="Path to the sgtranslator config.json file (default: ./config.json)"
    )
    parser.add_argument(
        "-f",
        "--filter",
        default=None,
        help="Optional: Run a specific test by name (without extension)"
    )
    parser.add_argument(
        "--ignore-stderr",
        action="store_true",
        help="Don't compare stderr between ARM and x86 outputs"
    )
    parser.add_argument(
        "--cc-arg",
        nargs=argparse.REMAINDER,
        default=[],
        help="Extra compiler arguments passed to aarch64-unknown-linux-gnu-gcc. Must be last option."
    )
    parser.add_argument(
        "--keep-all",
        action="store_true",
        help="Keep all test directories (passed and failed). By default, only failed tests are kept."
    )
    
    args = parser.parse_args()
    
    # Set global option
    ignore_stderr = args.ignore_stderr
    
    # Resolve sgtranslator path
    sgtranslator_path = args.sgtranslator_path
    if sgtranslator_path is None:
        sgtranslator_path = shutil.which("sgtranslator")
        if sgtranslator_path is None:
            parser.error("sgtranslator not found in PATH. Specify with --sgtranslator-path")
    
    run_tests(args.test_dir, sgtranslator_path, args.config_path, args.filter, args.cc_arg, args.keep_all)