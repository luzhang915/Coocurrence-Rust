import matplotlib.pyplot as plt
import sys

def ploter1(xs, ys):
    plt.plot(xs, ys)  
    plt.xticks(" ")
    plt.yticks(" ")
    plt.ylabel("num_coocurrence")
    plt.xlabel("win_size")
    plt.show()

def ploter_multiple(xs, y1, y2, y3, y4, y5):
	label1 = 'Sherlock-Holmes-Watson'
	label2 = '221B-Baker-Street'
	label3 = 'deceptive-obvious-fact'
	label4 = 'do-you-see'
	label5 = 'dear-Watson-I'
	plt.plot(xs, y1, 'r-', xs, y2, 'b-', xs, y3, 'g-', xs, y4, 'y-', xs, y5, 'k-')
	plt.xticks(" ")
	plt.yticks(" ")
	plt.ylabel("num_coocurrence")
	plt.xlabel("win_size")
	plt.legend([label1,label2, label3, label4, label5])
	plt.show()

def reader(path):
	file = open(path, 'r') 
	lines = file.readlines() 
	return lines

def plot1():
	path = sys.argv[1]
	x = []
	y = []
	count = 0
	for line in reader(path):
		count+=1
		x.insert(0, line.split(' ')[0])
		y.insert(0, line.split(' ')[1])

	
	ploter1(x[0:100],y[0:100])

def plot_multiple():
	path1 = sys.argv[1]
	path2 = sys.argv[2]
	path3 = sys.argv[3]
	path4 = sys.argv[4]
	path5 = sys.argv[5]

	x = []
	y1 = []
	y2 = []
	y3 = []
	y4 = []
	y5 = []

	count = 0
	for line in reader(path1):
		count+=1
		x.insert(0, line.split(' ')[0])
		y1.insert(0, line.split(' ')[1])

	for line in reader(path2):
		y2.insert(0, line.split(' ')[1])

	for line in reader(path3):
		y3.insert(0, line.split(' ')[1])

	for line in reader(path4):
		y4.insert(0, line.split(' ')[1])

	for line in reader(path5):
		y5.insert(0, line.split(' ')[1])

	# ploter_multiple(x[0:100],y1[0:100], y2[0:100], y3[0:100], y4[0:100], y5[0:100])
	ploter_multiple(x, y1, y2, y3, y4, y5)


plot_multiple()