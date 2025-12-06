cords = []
folds = []
with open('d13\data.txt', 'r') as f:
    for l in f.readlines():
        if l.startswith('fold'):
            fold = int(l.strip()[13:])

            if l.strip()[11] == 'x':
                folds.append([fold, 0])
            else:
                folds.append([0, fold])

        elif l != '\n':
            row = l.strip().split(',')
            x = int(row[0])
            y = int(row[1])
            cords.append([x, y])

board = [['.' for _ in range(folds[0][0] * 2 + 1)]
         for _ in range(folds[1][1] * 2 + 1)]

for x, y in cords:
    board[y][x] = '#'


x, y = folds[0]
if x:
    new_board = [['.' for _ in range(x)] for _ in range(len(board))]

    for i, row in enumerate(new_board):
        for j, item in enumerate(row):
            new_board[i][j] = '#' if board[i][j] == '#' or board[i][len(
                board[0]) - 1 - j] == '#' else '.'
else:
    new_board = [['.' for _ in range(len(board[0]))] for _ in range(y)]

    for i, row in enumerate(new_board):
        for j, item in enumerate(row):
            new_board[i][j] = '#' if board[i][j] == '#' or board[len(
                board) - 1 - i][j] == '#' else '.'

board = new_board

count = 0
for row in board:
    for x in row:
        if x == '#':
            count += 1

print(count)
