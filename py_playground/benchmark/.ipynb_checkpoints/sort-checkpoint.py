import random
import time

nums = 1000000

a1 = []
a2 = []
for i in range(0, nums):
    t = random.randint(0, nums)
    a1.append(t)
    a2.append(t)

t1 = time.time()
a1.sort()
t2 = time.time()
a2.sort()
t3 = time.time()

print("t2-t1 %s " % (t2-t1))

print("t3-t2 %s " % (t3-t2))