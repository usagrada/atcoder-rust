def main():
    n = int(input())
    ax, ay = map(int, input().split())
    bx, by = map(int, input().split())
    s = []
    for i in range(n):
        si = input()
        s.append(si)
    
    if (ax + ay + bx + by) % 2 == 1:
        print(-1)
        return
    


if __name__ == '__main__':
    main()