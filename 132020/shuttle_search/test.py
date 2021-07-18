a, b = open("input").read().strip().split("\n")
timestamp = int(a)
busses = [(i, int(x)) for i, x in enumerate(b.split(",")) if x != "x"]

# Part 1.
min_wait = float("inf")
part1 = None

# Part 2.
d = 1
i = 0

print(busses)

for offset, bus in busses:
    # Part 1. 
    loops = -(timestamp // -bus)
    wait = loops * bus - timestamp
    if wait < min_wait:
        part1 = wait * bus
        min_wait = wait

    # Part 2.
    while True:
        i += d
        if (i + offset) % bus == 0:
            d = d * bus
            # once it multiply by this bus number, it will be divisible by it
            # and all the bus numbers that came bfore it
            print('d increment ', d, 'i increment', i)
            break

print(part1)
print(i)
