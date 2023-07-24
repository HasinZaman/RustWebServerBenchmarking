import os
import math

# Defining constants
MEMORY_USAGE = "MEMORY_USAGE"
REQUEST_TIME = "REQUEST_TIME"

# File parsing

def parse_row(data_type):
    match(data_type):
        case "MEMORY_USAGE":
            return lambda elem: (float(elem[0]), float(elem[1]))
        case "REQUEST_TIME":
            return lambda elem: (float(elem[0]), int(elem[1]), float(elem[2]))
    pass
def get_file_content(data_type, path: str) -> str:
    raw_data = ""
    
    with open(path, "r") as f:
        raw_data = f.read()
    
    parser = parse_row(data_type)

    return [parser(data.split(",")) for data in raw_data.split("\n")]

def get_file_classification(name: str):
    name_components = name[:-4].split("_")

    return ("_".join(name_components[:-2]).upper(),name_components[-2].upper(),name_components[-1])

def get_files():
    data = []
    for file in os.scandir("."):
        if not file.is_file():
            continue

        name = file.name

        if not name.endswith(".csv"):
            continue
        
        print("parsing:{0}".format(name))

        classification = get_file_classification(name)

        data.append((classification, get_file_content(classification[0], name)))
    return data

# File partitioning
def partition(seg, set):
    partition_1 = []
    partition_2 = []

    for elem in set:
        if seg(elem):
            partition_1.append(elem)
        else:
            partition_2.append(elem)
    
    return partition_1, partition_2

def debug_wrapper(func):
    def tmp(elem):
        print("input:{}".format(elem))
        result = func(elem)
        print("result:{}".format(result))
        return result
    return tmp

def is_mem(elem):
    return elem[0][0] == MEMORY_USAGE
def is_small(elem):
    return elem[0][1] == "SMALL"
def is_backend(backend_tag):
    return lambda elem: elem[0][2] == backend_tag

# Stats
get_key = {MEMORY_USAGE: lambda e: e[1], REQUEST_TIME: lambda e: e[2]}

def percentile(k: float, sorted_data):
    index = math.floor(len(sorted_data) * k)

    return sorted_data[index]

def remove_outliers(data, key = lambda x: x):
    sorted_data = sorted(data, key=key, reverse=True)

    percentile_25 = key(percentile(0.25, sorted_data))
    percentile_75 = key(percentile(0.75, sorted_data))

    quartile_range = abs(percentile_75-percentile_25)

    upper = percentile_75 + 1.5*quartile_range
    lower = percentile_25 - 1.5*quartile_range

    return partition(
        lambda elem: key(elem) < lower or upper < key(elem),
        request_small_flask[1]
    )[0]

# Graph data
# - Graph by type
# - Graph by group
# - waterfall graph of memory consumption with scatter graph on top


if __name__ == "__main__":
    files = get_files()

    print([e[1][:10] for e in files])
    print("")
    print("")
    print([e[0] for e in partition(is_mem, files)[0]])
    print("")
    print([e[0] for e in partition(is_mem, files)[1]])
    print("")

    print("")
    print([e[0] for e in partition(is_small, files)[0]])
    print("")
    print([e[0] for e in partition(is_small, files)[1]])
    print("")
    
    print("")
    print([e[0] for e in partition(is_backend("flask"), partition(is_small, files)[0])[0]])
    print([e[0] for e in partition(is_backend("flask"), partition(is_small, files)[0])[1]])