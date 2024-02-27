from datetime import datetime
from sys import argv

import requests

server = argv[1]
device_token = argv[2]


def test():
    run_id = ""
    print("waiting", end="")
    while run_id == "":
        print(".", end="")
        res = requests.get(f"{server}/que/pop", headers={"token": device_token})
        if res.status_code != 200:
            print(res.content.decode())
            return
        run_id = res.content.decode()
    print(run_id)
    input("waiting for input")
    start_time = int(datetime.now().timestamp())
    print(start_time)
    print(
        requests.post(
            f"{server}/data/start",
            headers={"token": device_token},
            data={"time": start_time, "id": run_id},
        ).content.decode()
    )
    input("waiting for input")
    finish_time = int(datetime.now().timestamp())
    print(finish_time)
    print(
        requests.post(
            f"{server}/data/finish",
            headers={"token": device_token},
            data={"time": finish_time, "id": run_id},
        ).content.decode()
    )


if __name__ == "__main__":
    test()
