
import time


def timer(n):
    def wrap(f):
        def inner(*args, **kwargs):
            print(f"Running {f.__name__} {n} times...")
            start_time = time.perf_counter()
            print(f"\rProgress: {0.0:.2f}%", end="")
            for i in range(n):
                f(*args, **kwargs)
                progress = 100 * (i + 1) / n
                time_per_call = (time.perf_counter() - start_time) / (i + 1)
                time_remaining = (n - i - 1) * time_per_call
                print(f"\rProgress: {progress:.2f}% Estimated time remaining: {time_remaining:.2f} s", end="")
        return inner

    return wrap
