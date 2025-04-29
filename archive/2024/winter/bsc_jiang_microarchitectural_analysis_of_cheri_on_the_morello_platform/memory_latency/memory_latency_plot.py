import matplotlib.pyplot as plt
import pandas as pd

NOCAP_CSV = '/memory_latency_nocap.csv'
HYRBID_CSV = '/memory_latency_hybrid.csv'
PURECAP_CSV = '/memory_latency_purecap.csv'
PLOT_SVG = '/memory_latency_plot.svg'

def main():
    nocap_data = pd.read_csv(NOCAP_CSV)
    hybrid_data = pd.read_csv(HYRBID_CSV)
    purecap_data = pd.read_csv(PURECAP_CSV)

    x_label, y_label = nocap_data.columns[0], nocap_data.columns[1]

    fig, ax = plt.subplots()

    ax.plot(nocap_data[x_label], nocap_data[y_label], label='nocap', linestyle='-')
    ax.plot(hybrid_data[x_label], hybrid_data[y_label], label='hybrid', linestyle='--')
    ax.plot(purecap_data[x_label], purecap_data[y_label], label='purecap', linestyle='-.')

    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)

    ax.set_xscale('log')

    ax.legend()

    ax.grid(True)

    plt.savefig(PLOT_SVG, format='svg')
    plt.close()

if __name__ == '__main__':
    main()
