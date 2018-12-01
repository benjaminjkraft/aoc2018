import sys
x=sys.stdin.read().split()
y=set()
z=0
for i in range(10000000):
    z+=int(x[i%len(x)])
    if z in y:
        print z
        break
    y.add(z)
