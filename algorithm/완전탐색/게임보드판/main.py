
##
# https://www.algospot.com/judge/problem/read/BOARDCOVER
# 
C = int(input())

coverTypes = [
    [(0,0), (0,1), (-1,1)],
 [(0,0),(1,0),(1,1)], 
 [(0,0),(1,0),(0,1)], 
 [(0,0),(0,1),(1,1)]
]


def boardCover():
    x, y = -1, -1

    for i in range(h):
        for j in range(w):
            if board[i][j]=='.':
                x = j
                y = i
                break
        if y != -1 :
            break
    if x == -1 and y == -1 : return 1

    count = 0
    for coverType in coverTypes:
        flag = True
        for dx, dy in coverType:
            ndx, ndy = dx + x, dy + y
            if ndx < 0 or ndx > w or ndy < 0 or ndy >= h:
                flag = False
                break
            if board[ndy][ndx] == '#':
                flag = False
                break
        if flag == True:
            for dx, dy in coverType:
                ndx = dx + x
                ndy = dy + y
                board[ndy][ndx] = '#'
            count += boardCover()
            for dx, dy in coverType:
                ndx = dx + x
                ndy = dy + y
                board[ndy][ndx] = '.'
    return count

for _ in range(C) :
    h, w = map(int, input().split())
    board = [list(input()) for _ in range(h)]
    print(boardCover())