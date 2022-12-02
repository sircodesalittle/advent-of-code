def part1():
    elf_with_most = 0
    count = 0
    with open('./data/day1.txt') as f:
        for line in f.readlines():
            if line.strip() == "":
                if count > elf_with_most:
                    elf_with_most = count
                count = 0
            else:
                count = int(line) + count
    print(elf_with_most)

def part2():
    count = 0
    elves_by_count = []
    with open('./data/day1.txt') as f:
        for line in f.readlines():
            if line.strip() == "":
                elves_by_count.append(count)
                count = 0
            else:
                count = int(line) + count
    sorted_elves = sorted(elves_by_count, reverse=True)
    print(sum(sorted_elves[0:3]))
            

if __name__ == '__main__':
    # Answer to day 1 part 1 is 69206
    # answer to day 1 part 2 is 197400
    part1()
    part2()