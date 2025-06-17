import csv
import os
import statistics

def analyze_speed_difference(file1_path, file2_path):
    data1 = {}
    data2 = {}
    speed_differences = []

    try:
        with open(file1_path, mode='r', newline='', encoding='utf-8') as infile:
            reader = csv.DictReader(infile)
            if 'filename' not in reader.fieldnames or 'lifting_time_ns' not in reader.fieldnames:
                print(f"Error: '{file1_path}' must contain 'filename' and 'lifting_time_ns' columns.")
                return None, None
            for row in reader:
                try:
                    data1[row['filename']] = int(row['lifting_time_ns'])
                except ValueError:
                    print(f"Warning: Non-integer lifting_time_ns found in {file1_path} for {row['filename']}. Skipping.")
                except KeyError:
                    print(f"Warning: Missing 'filename' or 'lifting_time_ns' in a row of {file1_path}. Skipping.")
    except FileNotFoundError:
        print(f"Error: The file '{file1_path}' was not found.")
        return None, None
    except Exception as e:
        print(f"An error occurred while reading '{file1_path}': {e}")
        return None, None

    try:
        with open(file2_path, mode='r', newline='', encoding='utf-8') as infile:
            reader = csv.DictReader(infile)
            if 'filename' not in reader.fieldnames or 'lifting_time_ns' not in reader.fieldnames:
                print(f"Error: '{file2_path}' must contain 'filename' and 'lifting_time_ns' columns.")
                return None, None
            for row in reader:
                try:
                    data2[row['filename']] = int(row['lifting_time_ns'])
                except ValueError:
                    print(f"Warning: Non-integer lifting_time_ns found in {file2_path} for {row['filename']}. Skipping.")
                except KeyError:
                    print(f"Warning: Missing 'filename' or 'lifting_time_ns' in a row of {file2_path}. Skipping.")
    except FileNotFoundError:
        print(f"Error: The file '{file2_path}' was not found.")
        return None, None
    except Exception as e:
        print(f"An error occurred while reading '{file2_path}': {e}")
        return None, None

    common_filenames = set(data1.keys()).intersection(set(data2.keys()))

    if not common_filenames:
        print("No common filenames found between the two files. Cannot calculate differences.")
        return None, None

    for filename in common_filenames:
        time1 = data1[filename]
        time2 = data2[filename]

        if time2 == 0:
            print(f"Warning: 'lifting_time_ns' for '{filename}' in '{file2_path}' is zero. Skipping ratio calculation for this entry.")
            continue

        ratio = time1 / time2
        speed_differences.append(ratio)

    if not speed_differences:
        print("No valid speed differences could be calculated.")
        return None, None

    average_diff = statistics.mean(speed_differences)
    median_diff = statistics.median(speed_differences)

    return average_diff, median_diff

if __name__ == "__main__":
    file1 = "benchmark_sightglass.csv"
    file2 = "benchmark_sightglass_objdump.csv"

    print(f"\nAnalyzing speed differences between '{file1}' (Lifter) and '{file2}' (Objdump)...\n")
    avg_ratio, med_ratio = analyze_speed_difference(file1, file2)

    if avg_ratio is not None and med_ratio is not None:
        print(f"Average speed ratio (Lifter / Objdump): {avg_ratio:.2f}")
        print(f"Median speed ratio (Lifter / Objdump): {med_ratio:.2f}")
    else:
        print("Could not complete analysis due to errors or lack of data.")
