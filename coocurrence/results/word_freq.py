import re

def read_to_array(path):
    file = open(path, 'r')
    data = file.read()
    p = re.compile(r'\W+')
    words = p.split(data)
    return words


def count(words):
    counter = {}
    for word in words:
        if word in counter:
            counter[word] += 1
        else:
            counter.update({word: 1})
    return counter


def binomial(n, p):
    return factorial(n)/(factorial(n-p)*factorial(p))


def factorial(num):
    fact = 1
    for i in range(1, num):
        fact *= i
    return fact


def main():
    words = read_to_array('../SherlockHolmes.txt')
    # words_group = ["Sherlock", "Holmes", "Watson"]
    # words_group = ["221B", "Baker", "Street"]
    words_group = ["deceptive", "obvious", "fact"]
    # words_group = ["do", "you", "see"]
    # words_group = ["dear", "Watson", "I"]
    size = len(words)
    counts = count(words)

    freqs = 1
    for word in words_group:
        freqs *= counts[word]

    for i in reversed(range(1,10000)):
        print(f'{i} {int(binomial(i,3) * freqs)}')


if __name__ == "__main__":
    main()