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
    i = 0
    last_winner = []
    while i < len(boards):
        won = False

        for y in boards[i]:
            if all(x < 0 for x in y):
                won = True

        for j in range(5):
            if all(y[j] < 0 for y in boards[i]):
                won = True

        if won:
            last_winner = boards.pop(i)
        else:
            i += 1

    return last_winner


def count_score(board, number):
    sum_ = 0
    for y in board:
        for x in y:
            if x > 0:
                sum_ += x

    return sum_ * number


def play():
    last_winner = []
    last_draw = -1
    for number in draw_order:
        mark_number(number)

        if winner := check_boards():
            last_winner = winner
            last_draw = number

    print(last_winner, last_draw)
    return count_score(last_winner, last_draw)


print(play())
