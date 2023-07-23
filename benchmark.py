from multiprocessing import Pool
import subprocess
import threading
import time
import requests
import docker
import importlib
import re


def create_image(name: str, dir: str):
    print("Creating Image:{0}".format(name))
    subprocess.check_output("docker build -t {0} {1}".format(name, dir))
    print("Created 🎂")

def start_container(name: str, port: int):
    print("Starting Container:{0}".format(name))
    subprocess.check_output("docker run -it --rm -d -p {1}:{1} --name {0} {0}".format(name, port))
    print("Started 🏃‍♂️")

def close_container():
    print("Closing containers")
    output = subprocess.check_output("docker ps -q")
    container_ids = output.split()
    for id in container_ids:
        subprocess.call(["docker", "stop", id])
    print("Closed 💀")

def send_request(url: str):
    start = time.time()

    try:
        response = requests.get(url) 
        
        end = time.time()
        elapsed = end - start

        return (start, response.status_code, elapsed)
    except:
        end = time.time()
        elapsed = end - start

        return (start, "TIMEOUT", elapsed)
    

def save_data(file_name: str, data: str):
    with open(file_name, "w") as f:
        f.write(data)

def bench_mark(image_name, output_name, url):
    request_time_data = []
    memory_usage = []
    
    def request_test():
        with Pool() as pool:
            print("Sending requests")
            
            threads = []

            for _i in range(10000):
                threads.append(pool.apply_async(send_request, [url]))
            
            while 0 < len(threads):
                value = threads[0].get()
                request_time_data.append(value)
                del threads[0]

            print("Done sending requests")

    def measure_memory():
        client = docker.from_env()

        containers = client.containers.list(all=True)

        container = None

        for c in containers:
            if c.name == image_name:
                container = c
                break
        print("Starting memory test")
        while True:
            try:
                stats = container.stats(stream=False)
                
                memory = stats["memory_stats"]["usage"]

                memory_usage.append((time.time(), memory / (1024 * 1024)))

                time.sleep(0.1)
            except:
                break

    try:
        thread = threading.Thread(target=measure_memory)
        thread.start()

        time.sleep(0.5)

        request_test()
    except  Exception as e:
        print("Error:{}".format(e.__repr__()))

    request_time_data.sort(key=lambda tup: tup[0])
    memory_usage.sort(key=lambda tup: tup[0])

    save_data(
        "benchmarkingResults\\request_time_{}.csv".format(output_name),
        "\n".join(
            map(
            lambda tup: ",".join(map(str, tup)),
            request_time_data
            )
        )
    )
    save_data(
        "benchmarkingResults\\memory_usage_{}.csv".format(output_name),
        "\n".join(
            map(
            lambda tup: ",".join(map(str, tup)),
            memory_usage
            )
        )
    )

def extract_test_name(name:str) -> str:
    match = re.search(r'^[a-z]+(?=[A-Z])', name)

    if match:
        first_word = match.group()
        return first_word
    else:
        return name

def test_dir(dir: str):
    dir_data = importlib.import_module("{}.files".format(dir.replace("/", ".").replace("\\", ".")))
    
    name = extract_test_name(dir)

    test_env = "{}testenv".format(name)
    
    create_image(test_env, dir)
    start_container(test_env, dir_data.port)

    print("starting tests")
    for file in dir_data.tests[:1]:
        bench_mark(test_env, name, "_".join(file))

    close_container()

if __name__ == "__main__":
    import os

    for entry in os.scandir("."):
        if not entry.is_dir():
            continue
        dir_name = entry.name
        
        file_path = os.path.join(dir_name, "files.py")

        if not os.path.exists(file_path):
            continue
            
        test_dir(dir_name)