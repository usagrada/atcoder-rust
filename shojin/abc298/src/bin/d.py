def main():
    q = int(input())
    s = "1"
    for _ in range(q):
        line = input()
        vals = list(map(int, line.split()))
        if vals[0] == 1:
            s += str(vals[1])
        elif vals[0] == 2:
            s = s[1:]
        elif vals[0] == 3:
            MOD = 998244353
            print(int(s) % MOD)
if __name__ == '__main__':
    main()
