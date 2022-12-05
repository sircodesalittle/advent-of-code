from string import ascii_letters
value_system = {}

def create_value_system(): 
    for index, letter in enumerate(ascii_letters, start=1):
        value_system[letter] = index


def part1():
    total_count = 0
    create_value_system()
    with open('./data/day3.txt') as f:
        for line in f.readlines():
            line = line.strip()
            middle = int(len(line) / 2)
            first_half = line[0:middle]
            second_half = line[middle:]
            intersection = list(set(first_half) & set(second_half))
            total_count = total_count + value_system[intersection[0]]
    print(total_count)

def part2():
    total_count = 0
    lines_to_group = []
    create_value_system()
    with open('./data/day3.txt') as f:
        for line in f.readlines():
            line = line.strip()
            lines_to_group.append(line)
            if len(lines_to_group) == 3:
                intersection = list(set(lines_to_group[0]) & set(lines_to_group[1]) & set(lines_to_group[2]))
                total_count = total_count + value_system[intersection[0]]
                lines_to_group = []
    print(total_count)
    



if __name__ == '__main__':
    # part1()
    part2()
    
