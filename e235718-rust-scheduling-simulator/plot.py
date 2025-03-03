import matplotlib.pyplot as plt
import matplotlib.dates as mdates
import csv

def create_gantt_chart(data, algorithm):
    fig, ax = plt.subplots(figsize=(10, 6))

    colors = plt.cm.tab10.colors

    for idx, task in enumerate(data):
        ax.barh(task['id'], task['finish_time'] - task['start_time'], left=task['start_time'], color=colors[idx % len(colors)])

    ax.set_xlabel('Time (in units)')
    ax.set_ylabel('Task ID')
    ax.set_title(f'Gantt Chart for {algorithm} Scheduling')

    ax.grid(True)

    plt.tight_layout()

    plt.savefig(f"./img/{algorithm}_guntt_plot.svg")

    plt.show()

def read_data_from_csv(file_name):
    tasks = []
    with open(file_name, mode='r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            task = {
                'id': int(row['id']),
                'start_time': int(row['start_time']),
                'finish_time': int(row['finish_time'])
            }
            tasks.append(task)
    return tasks

def main():
    # select "fifo" or "sjf" or "round_robin"
    algorithm = "round_robin"  

    file_name = f"./data/{algorithm}_data.csv"
    data = read_data_from_csv(file_name)

    create_gantt_chart(data, algorithm)

if __name__ == '__main__':
    main()
