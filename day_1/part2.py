f = open('input', 'r')

numbers = []
for l in f:
    numbers.append(int(l.strip()))

i = 0
for n in numbers:
    j = 0
    for o in numbers[i:]:   # slicing like this makes a copy I don't like this
        for p in numbers[i+j:]: # would be better to use a range and indices
            if o+n+p == 2020:
                print(o*n*p)
        j+=1
    i+=1
