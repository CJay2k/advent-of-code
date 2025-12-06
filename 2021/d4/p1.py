draw_order = []
boards = []

with open('d4\data.txt', 'r') as f:
    draw_order = map(int, f.readline().split(','))

    while f.readline():
        board = [[int(x) for x in f.readline().split()] for _ in range(5)]
        boards.append(board)


def mark_number(number):
    for board in boards:
        for y in board:
            for i in range(5):
                if y[i] == number:
                    y[i] -= 100


def check_boards():
    for board in boards:
        for y in board:
            if all(x < 0 for x in y):
                return board

        for i in range(5):
            if all(y[i] < 0 for y in board):
                return board

    return []


def count_score(board, number):
    sum_ = 0
    for y in board:
        for x in y:
            if x > 0:
                sum_ += x

    return sum_ * number


def play():
    for number in draw_order:
        mark_number(number)

        if winner := check_boards():
            return count_score(winner, number)


print(play())
