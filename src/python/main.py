N = 40
time = __import__("time")
start = time.time()

def fibonacci(n):
    if n < 2:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


counter = 0
while counter < N:
    print(f"{counter}: {fibonacci(counter)}")
    counter += 1

elapsed = time.time() - start
print("Elapsed time: {}ms ({})".format(elapsed * 1000, elapsed))
print("Memory usage: {} MB".format(__import__("resource").getrusage(__import__("resource").RUSAGE_SELF).ru_maxrss / 1000))
