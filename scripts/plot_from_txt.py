import matplotlib.pyplot as plt
import matplotlib.ticker as ticker
from matplotlib.widgets import Slider
import glob
import os
import numpy as np

folder_path = "output"
file_list = glob.glob(os.path.join(folder_path, "*.txt"))
all_data = []
titles = []
plots_widgets = []

for file_path in file_list:
    with open(file_path, encoding='utf-8') as f:
        lines = f.readlines()

    title = lines[0].strip("#").strip()
    data = list(map(float, lines[1:]))
    all_data.append(data)
    titles.append(title)

    fig, ax = plt.subplots()
    plt.subplots_adjust(bottom=0.25)

    bins = np.linspace(min(data), max(data), num=50)
    ax.hist(data, bins=bins, align='left', rwidth=0.8)
    ax.set_title(title)
    ax.set_xlabel("Value")
    ax.set_ylabel("Frequency")
    ax.grid(True)

    step_x_init = 1
    step_y_init = max(ax.get_yticks()) // 5 or 1

    ax.xaxis.set_major_locator(ticker.MultipleLocator(step_x_init))
    ax.yaxis.set_major_locator(ticker.MultipleLocator(step_y_init))
    ax.grid(True)

    axcolor = 'lightgoldenrodyellow'
    ax_stepx = plt.axes([0.15, 0.1, 0.65, 0.03], facecolor=axcolor)
    ax_stepy = plt.axes([0.15, 0.05, 0.65, 0.03], facecolor=axcolor)

    slider_stepx = Slider(ax_stepx, 'Step X', 0.1, 10.0, valinit=step_x_init, valstep=0.1)
    slider_stepy = Slider(ax_stepy, 'Step Y', 0.1, 10.0, valinit=step_y_init, valstep=0.1)

    # Отдельная функция обновления с замыканием текущего ax и слайдеров
    def make_update(ax, slider_x, slider_y):
        def update(val):
            ax.xaxis.set_major_locator(ticker.MultipleLocator(slider_x.val))
            ax.yaxis.set_major_locator(ticker.MultipleLocator(slider_y.val))
            ax.figure.canvas.draw_idle()
        return update

    update_func = make_update(ax, slider_stepx, slider_stepy)
    slider_stepx.on_changed(update_func)
    slider_stepy.on_changed(update_func)

    plots_widgets.append({
        'fig': fig,
        'ax': ax,
        'slider_x': slider_stepx,
        'slider_y': slider_stepy,
        'update_func': update_func,
    })



fig, ax = plt.subplots()
plt.subplots_adjust(bottom=0.25)

# Определить общие границы для bin-ов
min_val = min(min(data) for data in all_data)
max_val = max(max(data) for data in all_data)
bins = np.linspace(min_val, max_val, 50)  # можно изменить число интервалов под ваш формат данных

# Нарисовать все гистограммы с прозрачностью, чтобы видеть пересечения
for data, title in zip(all_data, titles):
    ax.hist(data, bins=bins, alpha=0.5, label=title)

ax.set_title("Сравнительная гистограмма всех файлов")
ax.set_xlabel("Value")
ax.set_ylabel("Frequency")
ax.legend()
ax.grid(True)

plt.show()
