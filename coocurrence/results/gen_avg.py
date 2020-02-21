import sys


out_path = "plot_time/freq/avg_num_update.txt"
upper_bound = 20


def reader(path):
    file = open(path, 'r')
    lines = file.readlines()
    print("read file: {}".format(path))
    return lines


def main():
    path = sys.argv[1]
    out = open(out_path, 'w+')
    counts = {}
    values = {}

    for line in reader(path):
        l = line.split(' ')
        key = int(l[0])
        if key > upper_bound:
            continue
        if key in values.keys():
            values[key] += float(l[1])
            counts[key] += 1
        else:
            values[key] = float(l[1])
            counts[key] = 1

    for k in sorted(values.keys()):
        if k > upper_bound:
            continue
        if counts[k] == 0:
            continue
        out.write("{} {} \n".format(k, values[k] / counts[k]))
    # for (w, t) in values.items():
    #     if counts[w] == 0:
    #         continue
    #     out.write("{} {}\n".format(w, t / counts[w]))


if __name__ == '__main__':
    main()
