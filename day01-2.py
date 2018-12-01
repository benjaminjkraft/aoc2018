import sys,itertools as i
y={}
z=0
for j in i.cycle(sys.stdin):
 z+=int(j)
 if z in y:print z;break
 y[z]=1
