import os
import math
import drawsvg as draw

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
    for file in os.scandir("benchmarkingResults"):
        if not file.is_file():
            continue

        name = file.name

        if not name.endswith(".csv"):
            continue
        
        print("parsing:{0}".format(name))

        classification = get_file_classification(name)

        data.append((classification, get_file_content(classification[0], "benchmarkingResults//{}".format(name))))
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
        data[1]
    )[0]

# Graph data

def clone_data(data):
    return (data[0], [e for e in data[1]])

def get_time_range(waterfall_data, memory_data):
    time_stamp = [e[0] for e in waterfall_data[1]] + [e[0] for e in memory_data[1]]

    return (min(time_stamp), max(time_stamp))

def create_grid(svg, graph_size, offset, lines, line_generation):
    start = [0,0]
    end = [0,0]
    
    delta = (graph_size[0] / lines[0], graph_size[1] / lines[1])

    for i1 in range(2):
        match i1:
            case 0:# vertical line
                start = [0, offset[1]]
                end = [0, graph_size[1] + offset[1]]
                
            case 1:# horizontal line
                start = [offset[0], 0]
                end = [offset[0] + graph_size[0], 0]
                
        
        for i2 in range(lines[i1] + 1):
            start[i1] = offset[i1] + i2 * delta[i1]
            end[i1] = offset[i1] +  i2 * delta[i1]

            print("{}->{}".format(start, end))

            svg.append(line_generation(start, end))
            pass
   

def create_graph_background(svg, svg_size, time_range, **graph_data):
    # colour
    colour = {"background":"#190623","text":"#f7edfc", "primary": "#412d4b", "secondary":"#b58fb0","accent":"#dd2260"}

    if "colour" in graph_data:
        input_colour = graph_data["colour"]

        for key in colour.keys():
            if key in input_colour:
                colour[key] = input_colour[key]

    # graph size
    graph_size = (180, 80)
    if "size" in graph_data:
        graph_size = graph_data["size"]
    else:
        graph_size = (svg_size[0]*.9, svg_size[1]*.9)
    offset = (0, 0)
    if "offset" in graph_data:
        offset = graph_data["offset"]
    else:
        offset = (svg_size[0]*.05, svg_size[1]*.05)
    offset = (offset[0], svg_size[1] - offset[1] - graph_size[1])

    # axis lines
    major = (10, 10)
    if "major" in graph_data:
        offset = graph_data["major"]

    minor = (5, 5)
    if "minor" in graph_data:
        offset = graph_data["minor"]
    
    # Adding background colour
    svg.append(draw.Rectangle(*(0, 0), *svg_size, fill=colour["background"]))

    # Create minor grid lines
    create_grid(
        svg,
        graph_size,
        offset,
        (minor[0] * major[0], minor[1] * major[1]),
        lambda start, end: draw.Line(*start, *end, stroke=colour["accent"], stroke_width=1, fill='none')
    )
    # Create major grid lines
    create_grid(
        svg,
        graph_size,
        offset,
        (major[0], major[1]),
        lambda start, end: draw.Line(*start, *end, stroke=colour["secondary"], stroke_width=2, fill='none')
    )
    # Create border grid lines
    create_grid(
        svg,
        graph_size,
        offset,
        (1, 1),
        lambda start, end: draw.Line(*start, *end, stroke=colour["primary"], stroke_width=3, fill='none')
    )

def create_graph(waterfall_data, memory_data, **graph_data):
    waterfall_data = clone_data(waterfall_data)
    memory_data = clone_data(memory_data)

    time_range = get_time_range(waterfall_data, memory_data)

    time_map = lambda e: e[0] - time_range[0]

    SVG_SIZE = (800, 400)

    svg = draw.Drawing(*SVG_SIZE)

    create_graph_background(svg, SVG_SIZE)
    # draw graph lines & axis
    # draw title

    # add waterfall data

    # add scatter data

    svg.save_svg('example.svg')

# create svg
# create readme with all data


# - Graph by type
# - Graph by group
# - waterfall graph of memory consumption with scatter graph on top


if __name__ == "__main__":
    files = get_files()

    #print([e[1][:10] for e in files])
    print("")

    request_flask = partition(is_backend("flask"), files)[0]

    small_flask = partition(is_small,request_flask)[0]

    #print([e for e in remove_outliers(request_small_flask[1], key=get_key["REQUEST_TIME"])][:10])

    print([e[0] for e in small_flask])

    create_graph(small_flask[1], small_flask[0])

