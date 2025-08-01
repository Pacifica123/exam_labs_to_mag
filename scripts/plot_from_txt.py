import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
import glob
import os

folder_path = "output"
file_list = glob.glob(os.path.join(folder_path, "*.txt"))

for file_path in file_list:
    with open(file_path, encoding='utf-8') as f:
        lines = f.readlines()

    title = lines[0].strip("#").strip()
    data = list(map(int, lines[1:]))

    plt.figure()  # создаём отдельную фигуру для каждого сюжета
    plt.hist(data, bins=range(min(data), max(data) + 2), align='left', rwidth=0.8)
    plt.title(title)
    plt.xlabel("Value")
    plt.ylabel("Frequency")
    plt.grid(True)

    ax = plt.gca()
    ax.xaxis.set_major_locator(ticker.MultipleLocator(1))

plt.tight_layout()
plt.show()  # вызываем show один раз, после создания всех фигур
