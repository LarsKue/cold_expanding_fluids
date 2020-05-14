
import sys
import numpy as np
from scipy import constants as consts
from matplotlib import pyplot as plt


N = 1
k = 1
T_bar = 1
m = 1
w_sq = k / m

K = np.identity(3)
k_B = 1


def V(x):
    return k * x * x / 2


@np.vectorize
def kinetic_theory(t, x):
    den_term = 2 * k_B * T_bar * (1 + w_sq * t ** 2)
    return N * (k / (consts.pi * den_term)) ** (3 / 2) * np.exp(-k * np.dot(x, x) / den_term)


def main(argv: list) -> int:

    ts = [0, 1, 2, 3, 4]
    x = np.linspace(-6, 6, 10000)

    for t in ts:
        y = kinetic_theory(t, x)
        plt.plot(x, kinetic_theory(t, x), label=f"t = {t}")

    plt.xlabel(r"$x_1$")
    plt.ylabel(r"$n(t, \vec{x})$")
    plt.legend()
    plt.show()
    return 0


if __name__ == "__main__":
    main(sys.argv)