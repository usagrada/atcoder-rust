
def check(start, end):
  cnt = 0
  ok = 0
  for i in range(start, end):
    if s[i] == ".":
      if cnt == k:
        return k + ok
      cnt+=1
    else:
      ok += 1
  return cnt + ok

################################################################
s= input()
k = int(input())

l = len(s)
if l-k <= 0:
  print(l)
  exit()

ans = 0
cnt = check(0, l)

note = []
cnt = 0
for i in range(l):
  if s[i] == "X":
    cnt+=1
  else:
    if cnt != 0:
      note.append(cnt)
    cnt = 0

note2 = []
cnt = 0
for i in range(l):
  if s[i] == ".":
    cnt+=1
  else:
    if cnt != 0:
      note2.append(cnt)
    cnt = 0

print(note, note2)
ans = 0
if s[0] == "X":
  le_a = len(note)
  le_b = len(note2)
  # starta = 0
  # startb = 0
  cnt = 0
  change = 0
  tmp = 0
  start_i
  for i in range(le_a):
    if note2[i] + change <= k:
      cnt += note[i]
      if i <= le_b:
        change += note2[i]
    else:
      tmp = k - note2[i]
      cnt += tmp
      start_i = i
      break
  ans = cnt
  cnt -= tmp
  for i in range(start_i, le_a):
    cnt-=

else:
  starta = 1
  startb = 0
  cnt = 0
  change = 0
  for i in range(le_a):
    if note2[i] + change < k:
      cnt += note[i]
      change += note2[i]

print(ans)