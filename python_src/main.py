import psutil
import time

# Start measuring memory and time before the loop
process = psutil.Process()
initial_memory = process.memory_info().rss
start_time = time.time()
list1 = []
# Your for loop here
for i in range(1000):
    for j in range(1000):
        for k in range(1000):
            list1.append(i + j + k)

# Measure memory and time after the loop
final_memory = process.memory_info().rss
end_time = time.time()

# Calculate memory usage and time taken
memory_usage = final_memory - initial_memory
time_taken = end_time - start_time
print(f"Memory usage: {memory_usage:,.2f} bytes")
print(f"Time taken: {time_taken:.2f} seconds")
