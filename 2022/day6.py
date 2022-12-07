
def check_if_4_unique(current):
    last_four = current[-4:]
    if len(current) < 4:
        return False
    if last_four.count(current[-1]) == 1 and last_four.count(current[-2]) == 1 and last_four.count(current[-3]) == 1 and last_four.count(current[-4]) == 1:
        return True
    return False


def part1():
    with open('./data/day6.txt') as f:
        for line in f.readlines():
            line = line.strip()
            buffer = ''
            for character in line:
            # for character in 'mjqjpqmgbljsphdztnvjfqwrcgsmlb':
                buffer += character
                if check_if_4_unique(buffer):
                    print(len(buffer))
            

def check_if_14_unique(current):
    last_fourteen = current[-14:]
    if len(current) < 14:
        return False
    if last_fourteen.count(current[-1]) == 1 and \
        last_fourteen.count(current[-2]) == 1 and \
        last_fourteen.count(current[-3]) == 1 and \
        last_fourteen.count(current[-4]) == 1 and \
        last_fourteen.count(current[-5]) == 1 and \
        last_fourteen.count(current[-6]) == 1 and \
        last_fourteen.count(current[-7]) == 1 and \
        last_fourteen.count(current[-8]) == 1 and \
        last_fourteen.count(current[-9]) == 1 and \
        last_fourteen.count(current[-10]) == 1 and \
        last_fourteen.count(current[-11]) == 1 and \
        last_fourteen.count(current[-12]) == 1 and \
        last_fourteen.count(current[-13]) == 1 and \
        last_fourteen.count(current[-14]) == 1:
        return True
    return False

def part2():
    with open('./data/day6.txt') as f:
        for line in f.readlines():
            line = line.strip()
            buffer = ''
            for character in line:
            # for character in 'mjqjpqmgbljsphdztnvjfqwrcgsmlb':
                buffer += character
                if check_if_14_unique(buffer):
                    print(len(buffer))
            

if __name__ == '__main__':
    # part1()
    part2()
    
