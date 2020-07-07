
import numpy as np
from matplotlib import pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from grid_solver import GridSolver
from copy import deepcopy
from progress_timer import timer


def gaussian(x, y, z, x0, y0, z0, sigx, sigy, sigz, amplitude=1.0):
    return amplitude * np.exp(-0.5 * (((x - x0) / sigx) ** 2 + ((y - y0) / sigy) ** 2 + ((z - z0) / sigz) ** 2))


def gross_pitaevskii(m, l):

    def update_func(_, d):
        gs, ggs = GridSolver.gradients(d)
        delta_phi = ggs[0][0] + ggs[1][1] + ggs[2][2]
        return 1j * delta_phi / (2 * m) - 1j * l * np.abs(d) ** 2 * d / 6

    N = 100

    x = y = z = np.linspace(-8, 8, N)

    x, y, z = np.meshgrid(x, y, z)

    # we use a non-normalized gaussian in 3 dimensions
    # expected values
    x0 = 0
    y0 = 0
    z0 = 0
    # standard deviations
    sigx = 1
    sigy = 1
    sigz = 1

    data = gaussian(x, y, z, x0, y0, z0, sigx, sigy, sigz).astype(np.complex128)

    fig = plt.figure(figsize=(10, 10))
    ax = fig.add_subplot(111, projection="3d")
    ax.plot_surface(x[:, :, N // 2], y[:, :, N // 2], np.abs(data[:, :, N // 2]) ** 2, cmap="hot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")
    ax.set_zlim(0, 1)
    plt.savefig("t=0.png")
    plt.title(f"Absolute Squared Distribution at z = {z[0,0,N // 2]}")

    plt.show()

    g = GridSolver(deepcopy(data))

    h = 1
    n = 128

    @timer(n)
    def steps():
        g.step_rk4(update_func, 0, h)

    steps()

    fig = plt.figure(figsize=(10, 10))
    ax = fig.add_subplot(111, projection="3d")
    ax.plot_surface(x[:, :, N // 2], y[:, :, N // 2], np.abs(g.data[:, :, N // 2]) ** 2, cmap="hot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_zlabel("z")
    ax.set_zlim(0, 1)
    plt.savefig(f"t={n*h:.4f}.png")
    plt.title(f"Absolute Squared Distribution at z = {z[0, 0, N // 2]}")

    plt.show()


def main(argv: list) -> int:
    gross_pitaevskii(1, 1)
    return 0


if __name__ == "__main__":
    import sys
    main(sys.argv)
