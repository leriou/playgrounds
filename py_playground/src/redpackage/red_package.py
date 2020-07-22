import random


def gen_red_package(amount, num, min_size, max_size):
    amount = amount * 100
    lines = [0, amount]
    # 生成num-1个符合要求的随机的点
    for i in range(0, num-1):
        loop = True
        while loop:
            r = random.Random().randint(0, amount)
            loop = not is_valid(lines, r, min_size * 100, max_size * 100)
    paks = []
    for j in range(0, len(lines) - 1):
        paks.append((lines[j+1] - lines[j])/100)
    return paks


def is_valid(arr, value, min_size, max_size):
    if arr.__contains__(value):
        return False
    arr.append(value)
    arr.sort()
    curr = arr.index(value)
    l = arr[max(0, curr - 1)]
    r = arr[min(curr + 1, len(arr) - 1)]
    dl = value - l
    dr = r - value
    if dl >= min_size and dr >= min_size and dl <= max_size and dr <= max_size:
        return True
    arr.remove(value)
    return False


print(gen_red_package(100, 10, 5, 90))
