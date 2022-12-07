import re

# [Q]         [N]             [N]    
# [H]     [B] [D]             [S] [M]
# [C]     [Q] [J]         [V] [Q] [D]
# [T]     [S] [Z] [F]     [J] [J] [W]
# [N] [G] [T] [S] [V]     [B] [C] [C]
# [S] [B] [R] [W] [D] [J] [Q] [R] [Q]
# [V] [D] [W] [G] [P] [W] [N] [T] [S]
# [B] [W] [F] [L] [M] [F] [L] [G] [J]
#  1   2   3   4   5   6   7   8   9 

stack1 = ['B', 'V', 'S', 'N', 'T', 'C', 'H', 'Q']
stack2 = ['W', 'D', 'B', 'G']
stack3 = ['F', 'W', 'R', 'T', 'S', 'Q', 'B']
stack4 = ['L', 'G', 'W', 'S', 'Z', 'J', 'D', 'N']
stack5 = ['M', 'P', 'D', 'V', 'F']
stack6 = ['F', 'W', 'J']
stack7 = ['L', 'N', 'Q', 'B', 'J', 'V']
stack8 = ['G', 'T', 'R', 'C', 'J', 'Q', 'S', 'N']
stack9 = ['J', 'S', 'Q', 'C', 'W', 'D', 'M']
crane_stacks = [stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9]

def do_move(number, from_index, to_index):
    stack_to_move = []
    for x in range(0, number):
        stack_to_move.append(crane_stacks[from_index - 1].pop())
    stack_to_move.reverse()
    for crate in stack_to_move:
        crane_stacks[to_index - 1].append(crate)
    print()

def part1():
    with open('./data/day5.txt') as f:
        for line in f.readlines():
            line = line.strip()
            number, from_index, to_index = re.findall(r'\d+', line)
            do_move(int(number), int(from_index), int(to_index))
    final_value = ''
    for stack in crane_stacks:
        final_value += stack[-1]
    print(final_value)


def part2():
    with open('./data/day5.txt') as f:
        for line in f.readlines():
            line = line.strip()
            number, from_index, to_index = re.findall(r'\d+', line)
            do_move(int(number), int(from_index), int(to_index))
    final_value = ''
    for stack in crane_stacks:
        final_value += stack[-1]
    print(final_value)



if __name__ == '__main__':
    # part1()
    part2()
    
