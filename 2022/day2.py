def rock_paper_scissors(p1, p2):
    print(' ')
    print(p1 + ' : ' + p2)
    p1 = p1.strip()
    p2 = p2.strip()
    if p1 == 'A':
        p1 = 'rock'
    if p1 == 'B':
        p1 = 'paper'
    if p1 == 'C':
        p1 = 'scissors'
    if p2 == 'X':
        p2 = 'rock'
    if p2 == 'Y':
        p2 = 'paper'
    if p2 == 'Z':
        p2 = 'scissors'
    print(p1 + ' : ' + p2)
    if (p1 == 'rock' and p2 == 'rock') or (p1 == 'paper' and p2 == 'paper') or (p1 == 'scissors' and p2 == 'scissors'):
        return 'draw'
    if (p1 == 'rock' and p2 == 'scissors') or (p1 == 'scissors' and p2 == 'paper') or (p1 == 'paper' and p2 == 'rock'):
        return 'player1'
    if (p2 == 'rock' and p1 == 'scissors') or (p2 == 'scissors' and p1 == 'paper') or (p2 == 'paper' and p1 == 'rock'):
        return 'player2'

def get_points_for_round(their_choice, my_choice):
    result = rock_paper_scissors(their_choice, my_choice)
    if result == 'draw':
        print('draw')
        return 3
    elif result == 'player1':
        print('lose')
        return 0
    elif result == 'player2':
        print('win')
        return 6


def part1():
    total_points = 0
    with open('./data/day2.txt') as f:
        for line in f.readlines():
            opponent_choice, my_choice = line.split(' ')
            my_choice = my_choice.strip()
            print(opponent_choice)
            print(my_choice)
            if my_choice == 'X':
                print('+1')
                total_points = total_points + 1
            if my_choice == 'Y':
                print('+2')
                total_points = total_points + 2
            if my_choice == 'Z':
                print('+3')
                total_points = total_points + 3
            scored_points = get_points_for_round(opponent_choice, my_choice)
            print(scored_points)
            total_points = total_points + scored_points
            print('')
    print(total_points)

def determine_what_to_get(opponent_choice, outcome):
    if opponent_choice == 'A':
        opponent_choice = 'rock'
        if outcome == 'X': # lose
            return 'scissors'
        elif outcome == 'Y':
            return 'rock'
        else:
            return 'paper'
    elif opponent_choice == 'B':
        opponent_choice = 'paper'
        if outcome == 'X': # lose
            return 'rock'
        elif outcome == 'Y':
            return 'paper'
        else:
            return 'scissors'
    elif opponent_choice == 'C':
        opponent_choice = 'scissors'
        if outcome == 'X': # lose
            return 'paper'
        elif outcome == 'Y':
            return 'scissors'
        else:
            return 'rock'

def part2():
    total_points = 0
    with open('./data/day2.txt') as f:
        for line in f.readlines():
            opponent_choice, outcome = line.split(' ')
            outcome = outcome.strip()
            my_choice = determine_what_to_get(opponent_choice, outcome)
            if my_choice == 'rock':
                print('+1')
                total_points = total_points + 1
            if my_choice == 'paper':
                print('+2')
                total_points = total_points + 2
            if my_choice == 'scissors':
                print('+3')
                total_points = total_points + 3
            scored_points = get_points_for_round(opponent_choice, my_choice)
            print(scored_points)
            total_points = total_points + scored_points
            print('')
    print(total_points)
            

if __name__ == '__main__':
    # part1()
    part2()