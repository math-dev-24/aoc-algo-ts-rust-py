import requests
from dotenv import load_dotenv
import os

load_dotenv()
SESSION_TOKEN = os.getenv("SESSION")
BASE_URL = os.getenv("URL")


def post_data(year: int, day: int, data: str, level: int = 1):
    url = f"{BASE_URL}{year}/day/{day}/answer"
    headers = {"Cookie": f"session={SESSION_TOKEN}", "Content-Type": "application/x-www-form-urlencoded"}
    payload = {"level": level, "answer": data}

    try:
        response = requests.post(url, headers=headers, data=payload)
        response.raise_for_status()
        content = response.text.strip()
        if "That's not the right answer;" in content:
            return False
        return True
    except requests.exceptions.RequestException as e:
        return str(e)
