import random

alphabets = list(map(chr, range(65, 91)))
output_path = "plot_time/random_itemset_large.txt"
num = 1000
itemset_size = 50


def gen_rand_char_set(size, alphabets):
    itemset = set()
    while len(itemset) < size:
        c = random.choice(alphabets)
        itemset.add(c)
    return itemset


def gen_char_sets(num, alphabets, size, out):
    itemsets = []
    while len(itemsets) < num:
        itemset = gen_rand_char_set(size, alphabets)
        set = list(itemset).sort()
        itemsets.append(set)
        for i in itemset:
            out.write("{}".format(i))
        out.write(" \n")


def gen_char_sets_run():
    out = open(output_path, 'w+')
    for s in range(3, itemset_size+1):
        gen_char_sets(num, alphabets, s, out)


def check():
    counter = {}
    for s in range(3, itemset_size+1):
        counter[s] = 0
    for line in reader(output_path):
        counter[len(line.split(' ')[0])] += 1
    print(counter)


def reader(path):
    file = open(path, 'r')
    lines = file.readlines()
    return lines


if __name__ == '__main__':
    gen_char_sets_run()
    check()

