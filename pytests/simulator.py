import requests

SIMULATOR_URL = "http://localhost:13013"


def simulator_load_flash_image(path):
    with open(path, "rb") as f:
        return requests.post(f"{SIMULATOR_URL}/upload-flash-image", files={"image": f})


def link_create() -> int:
    return requests.post(f"{SIMULATOR_URL}/link/new").json()


def qemu_create_instance() -> int:
    return requests.post(f"{SIMULATOR_URL}/qemu/new").json()


def qemu_set_config_word(device: int, word_id: int, value: int):
    return requests.post(
        f"{SIMULATOR_URL}/qemu/{device}/config",
        json={"word_id": word_id, "value": value},
    ).json()


def qemu_connect_link_output(device: int, link_id: int):
    return requests.post(
        f"{SIMULATOR_URL}/qemu/{device}/connect",
        json={"link_id": link_id},
    ).json()


def simulator_launch():
    return requests.post(f"{SIMULATOR_URL}/launch").json()
