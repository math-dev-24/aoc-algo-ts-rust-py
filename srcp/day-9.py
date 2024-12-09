import time

from srcp.utils.input import get_data


def parse_disk_map(disk):
    tmp_disk = []
    index = 0
    for i, length in enumerate(map(int, disk)):
        if i % 2 == 0:
            tmp_disk.extend([i // 2] * length)
            index += 1
        else:
            tmp_disk.extend(["."] * length)
    return tmp_disk, index - 1


def move_whole_files(disk_data, quantity: int):
    for file_id in range(quantity, -1, -1):
        file_positions = [i for i, block in enumerate(disk_data) if block == file_id]
        if not file_positions:
            continue
        file_size = len(file_positions)
        #print(f"File {file_id} size : {file_size}")

        free_start = None
        #print(f"Index start : {index_start}")
        #print(f"File positions : {file_positions}")
        for i in range(file_positions[-1]):
            list_blocks = disk_data[i:i + file_size]
            if all(block == "." for block in list_blocks) and len(list_blocks) == file_size:
                # print(i)
                free_start = i
                break

        if free_start is not None:
            #print(f" move {file_id}")
            for pos in file_positions:
                disk_data[pos] = "."
            for j in range(file_size):
                disk_data[free_start + j] = file_id
            # print(disk_data)

    return disk_data


time_start = time.time()
# disk_map = "2333133121414131402"
disk_map = get_data(2024, 9)
disk_data, quantity = parse_disk_map(disk_map)
# print("Disk data :", disk_data)
# print("Quantity :", quantity)

compacted_data = move_whole_files(disk_data, quantity)
# print("Compacted data :", compacted_data)

checksum = sum(i * block for (i, block) in enumerate(compacted_data) if block != ".")

print("Resultat :", checksum)
print(f"Time : {time.time() - time_start:.2f}s")
