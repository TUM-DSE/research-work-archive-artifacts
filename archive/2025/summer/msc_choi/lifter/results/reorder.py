import csv
import os

def reorder_csv_alphabetically(input_file_path):
    try:
        with open(input_file_path, mode='r', newline='', encoding='utf-8') as infile:
            reader = csv.reader(infile)
            header = next(reader)
            data = []
            for row in reader:
                if row:
                    data.append(row)

        data.sort(key=lambda x: x[0].lower())

        with open(input_file_path, mode='w', newline='', encoding='utf-8') as outfile:
            writer = csv.writer(outfile)
            writer.writerow(header)
            writer.writerows(data)

    except FileNotFoundError:
        pass
    except StopIteration:
        pass
    except Exception:
        pass

if __name__ == "__main__":
    target_csv_file = "benchmark_sightglass_objdump.csv"
    reorder_csv_alphabetically(target_csv_file)
