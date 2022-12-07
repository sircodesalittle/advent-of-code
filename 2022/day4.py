def check_if_fully_contain(index1_start, index1_end, index2_start, index2_end):
    # test_str = range(0, 110)
    # list1 = test_str[index1_start: index1_end]
    # list2 = test_str[index2_start:index2_end]
    # if set(list1) <= set(list2) or set(list2) <= set(list1):
    #     return True

    if index1_start in range(index2_start, index2_end + 1) and index1_end in range(index2_start, index2_end + 1):
        return True
    if index2_start in range(index1_start, index1_end + 1) and index2_end in range(index1_start, index1_end + 1):
        return True

    # if index2_start in range(index1_start - 1, index1_end + 1) and index2_end in range(index1_start - 1, index1_end + 1):
    #     return True
    # if index1_start in range(index2_start - 1, index2_end + 1) and index1_end in range(index2_start - 1, index2_end + 1):
    #     return True
    return False

def part1():
    total_count = 0
    with open('./data/day4.txt') as f:
        for line in f.readlines():
            line = line.strip()
            assignment1, assignment2 = line.split(',')
            assignment1_index_1, assignment1_index_2 = assignment1.split('-')
            assignment2_index_1, assignment2_index_2 = assignment2.split('-')
            assignment1_index_1 = int(assignment1_index_1)
            assignment1_index_2 = int(assignment1_index_2)
            assignment2_index_1 = int(assignment2_index_1)
            assignment2_index_2 = int(assignment2_index_2)
            fully_contains = check_if_fully_contain(assignment1_index_1, assignment1_index_2, assignment2_index_1, assignment2_index_2)
            if fully_contains:
                total_count = total_count + 1
    print(total_count)


def check_if_partially_contain(index1_start, index1_end, index2_start, index2_end):
    # test_str = range(0, 110)
    # list1 = test_str[index1_start: index1_end]
    # list2 = test_str[index2_start:index2_end]
    # if set(list1) <= set(list2) or set(list2) <= set(list1):
    #     return True

    if index1_start in range(index2_start, index2_end + 1):
        return True
    if index1_end in range(index2_start, index2_end + 1):
        return True
    if index2_start in range(index1_start, index1_end + 1):
        return True
    if index2_end in range(index1_start, index1_end + 1):
        return True

    # if index2_start in range(index1_start - 1, index1_end + 1) and index2_end in range(index1_start - 1, index1_end + 1):
    #     return True
    # if index1_start in range(index2_start - 1, index2_end + 1) and index1_end in range(index2_start - 1, index2_end + 1):
    #     return True
    return False

def part2():
    total_count = 0
    with open('./data/day4.txt') as f:
        for line in f.readlines():
            line = line.strip()
            assignment1, assignment2 = line.split(',')
            assignment1_index_1, assignment1_index_2 = assignment1.split('-')
            assignment2_index_1, assignment2_index_2 = assignment2.split('-')
            assignment1_index_1 = int(assignment1_index_1)
            assignment1_index_2 = int(assignment1_index_2)
            assignment2_index_1 = int(assignment2_index_1)
            assignment2_index_2 = int(assignment2_index_2)
            does_contain = check_if_partially_contain(assignment1_index_1, assignment1_index_2, assignment2_index_1, assignment2_index_2)
            if does_contain:
                total_count = total_count + 1
    print(total_count)

if __name__ == '__main__':
    # part1() # between 474 and 608
    part2()
    
