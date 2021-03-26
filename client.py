import requests
import json


def submit(item):
    r = requests.post("http://localhost:3030/add", json=item)
    if r.text != "Added task":
        print(f"Submission failed! {r.text}")

def get_results():
    r = requests.get("http://localhost:3030/results")
    return json.loads(r.text)

if __name__ == "__main__":
    for i in range(10):
        item = {
            "state": i,
            "target_hash": "meow"
        }
        print(f"Submitting {item}")
        print(f"Submit success?: {submit(item)}")

    for res in get_results():
        print(res)