import matplotlib.pyplot as plt
import sys
import random



def reader(path):
    file = open(path, 'r')
    lines = file.readlines()
    return lines

def gen_color():
    r = random.randrange(0,10,1) /100
    g = random.randrange(0,10,1) /100
    b = random.randrange(0,10,1) /100
    return [r,g,b]

def plot_multi(xs, ys, labels):
    ax = plt.figure(figsize=(7, 4)).add_subplot(111)
    plt.setp(ax.spines.values(), linewidth=1.5)
    for y in ys:
        color = gen_color()
        ax.plot(xs, y, '-', color=(color[0], color[1], color[2]))
    plt.ylabel("num_coocurrence")
    plt.xlabel("win_size")
    plt.minorticks_on()
    plt.grid(color='grey', which='major', axis='y', linestyle='--')
    plt.grid(color='grey', which='major', axis='x', linestyle='--')
    plt.subplots_adjust(left=0.17, bottom=0.14, right=0.95, top=0.93,
                        wspace=0.2, hspace=0.2)
    # plt.legend(labels)
    plt.savefig("top10.pdf")

def main():
    paths = sys.argv[1:]
    xs = [i for i in range(1,2001)]
    ys = []
    labels = []

    for path in paths:
        y = []
        for line in reader(path):
            y.insert(0, int(line.split(' ')[1]))
        ys.append(y)
        labels.append(path)
    plot_multi(xs, ys, labels)

if __name__ == '__main__':
    main()