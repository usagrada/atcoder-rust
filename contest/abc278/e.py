def main():
    H, W, N, h, w = map(int, input().split())
    a = []
    for _ in range(H):
        a.append(list(map(int, input().split())))

    m = {}
    for i in range(H):
        for j in range(W):
            if a[i][j] in m:
                m[a[i][j]] += 1
            else:
                m[a[i][j]] = 1
    m2 = m.copy()
    for i in range(h):
        for j in range(w):
            m2[a[i][j]] -= 1

    ans_vec = [{} for _ in range(W - w + 1)]
    ans_vec[0] = m2.copy()

    for j in range(1, W-w+1):
        ans_vec[j] = ans_vec[j-1].copy()
        for k in range(h):
            ans_vec[j][a[k][j-1]] += 1
            ans_vec[j][a[k][j+w-1]] -= 1

    cnts = [0 for _ in range(W - w + 1)]
    for i in range(W - w + 1):
        d = ans_vec[i]
        for k, v in d.items():
            if v == 0:
                continue
            cnts[i] += 1
    print(*cnts)

    for i in range(1, H-h+1):
        for j in range(W-w+1):
            for k in range(w):
                ans_vec[j][a[i-1][j+k]] += 1
                ans_vec[j][a[i+h-1][j+k]] -= 1

        cnts = [0 for _ in range(W - w + 1)]
        for i in range(W - w + 1):
            d = ans_vec[i]
            for k, v in d.items():
                if v == 0:
                    continue
                cnts[i] += 1
        print(*cnts)


if __name__ == '__main__':
    main()
