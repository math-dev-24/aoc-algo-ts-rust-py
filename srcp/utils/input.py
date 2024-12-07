import requests
from dotenv import load_dotenv
import os


load_dotenv()
SESSION = os.getenv("SESSION")
URL = os.getenv("URL")


def get_data(year: int, day: int):
    url = f"{URL}/{year}/day/{day}/input"
    headers = {"Cookie": f"session={SESSION}"}

    try:
        response = requests.get(url, headers=headers)
    except requests.exceptions.RequestException as e:
        print(e)
        return None
    return response.text.strip()
