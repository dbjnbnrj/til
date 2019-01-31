
f = open('day2.txt', 'r')
words = [line for line in f.readlines()]
d = {}
wlen = len(words[0])
wordslen = len(words)

for k1 in range(wlen):
    d = {}
    for i in range(wordslen):
        s = str(words[i][0:k1]+words[i][k1+1:wlen])
        if s in d:
            print(s)
        d[s] = True
f.close()
