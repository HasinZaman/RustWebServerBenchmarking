from multiprocessing import Pool
import subprocess
import threading
import time
import requests
import docker

def create_image(name: str, dir: str):
    print("Creating Image:{0}".format(name))
    subprocess.check_output("docker build -t {0} {1}".format(name, dir))
    print("Created 🎂")

def start_container(name: str):
    print("Starting Container:{0}".format(name))
    subprocess.check_output("docker run -it --rm -d -p 8081:8081 --name {0} {0}".format(name))
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

    response = requests.get(url)

    end = time.time()
    
    elapsed = end - start
    
    return (start, response.status_code, elapsed)

def save_data(file_name: str, data: str):
    with open(file_name, "w") as f:
        f.write(data)

def bench_mark(image_name, url):
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

                time.sleep(0.5)
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
        "benchmarkingResults\\request_time_{}.csv".format(image_name),
        "\n".join(
            map(
            lambda tup: ",".join(map(str, tup)),
            request_time_data
            )
        )
    )
    save_data(
        "benchmarkingResults\\memory_usage_{}.csv".format(image_name),
        "\n".join(
            map(
            lambda tup: ",".join(map(str, tup)),
            memory_usage
            )
        )
    )

if __name__ == "__main__":
    create_image("flasktestenv", "flaskCustomImage")
    start_container("flasktestenv")

    bench_mark("flasktestenv", "http://localhost:8081/small")

    close_container()