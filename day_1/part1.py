f = open('input', 'r')

numbers = []
for l in f:
    numbers.append(int(l.strip()))

i = 0
for n in numbers:
    for o in numbers[i:]:
        if o+n == 2020:
            print(o*n)
    i+=1
