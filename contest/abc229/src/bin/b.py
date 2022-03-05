(a, b) = input().split()
al = len(a)
bl = len(b)

l = min(al, bl)

flag = False
for i in range(l):
  ai = int(a[al-i-1])
  bi = int(b[bl-i-1])
  # print(ai, bi)
  if ai + bi > 9:
    flag = True
    break
  
  
if flag:
  print("Hard")
else:
  print("Easy")