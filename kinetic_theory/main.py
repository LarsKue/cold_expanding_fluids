
from matplotlib import pyplot as plt
from random import normalvariate as nv
from particles import Particles, Vec3
import pathlib as pl

import time


def init_sgaussian(ps, n):
    for _ in range(n):
        position = Vec3(
            nv(0.0, 10.0),
            nv(0.0, 10.0),
            0.0
        )

        velocity = Vec3(
            nv(0.0, 1.0),
            nv(0.0, 1.0),
            0.0
        )

        mass = 1.0

        ps.add_particle(position, velocity, mass)


def main(argv: list) -> int:

    n_particles = 500
    n_steps = 5_001
    h = 0.01

    images_pathname = f"images_{n_particles}/"
    images_path = pl.Path(images_pathname)
    images_path.mkdir(parents=True, exist_ok=True)

    ps = Particles()

    init_sgaussian(ps, n_particles)

    def V(p):
        kx = 1 / 100
        ky = 3 / 100
        kz = 1 / 100

        factor = -(kx * p.x ** 2 + ky * p.y ** 2 + kz * p.z ** 2) / 2
        unit = p.unit()

        # PyO3 has a bug here that prevents us from using vector-scalar multiplication
        # so perform it manually in Python
        return Vec3(unit.x * factor, unit.y * factor, unit.z * factor)

    ps.set_potential(V)

    start_time = time.time()
    passed = None

    # TODO: Impulsverteilung, Zeitschritt logarithmisch (größer nach hinten)
    #  Excentricity über impulsverteilung

    for i in range(n_steps):
        # if i < 30 or i % 10 == 0:
            # plot all low times and every 10 steps for high times
        print(f"\rsaving image for t = {h * i}...", end="")
        positions = ps.positions()
        fig, axes = plt.subplots(nrows=2, ncols=2, figsize=(20, 16))
        axes[0][0].plot([p.x for p in positions], [p.y for p in positions], lw=0, marker=".")
        axes[0][0].set_xlabel("x")
        axes[0][0].set_ylabel("y")
        axes[0][0].set_title("XY Particle Distribution")
        axes[0][0].set_aspect("equal", adjustable="datalim")
        axes[0][1].hist2d([p.x for p in positions], [p.y for p in positions], bins=50)
        axes[0][1].set_xlabel("x")
        axes[0][1].set_ylabel("y")
        axes[0][1].set_title("XY Particle Density")
        axes[0][1].set_aspect("equal", adjustable="datalim")
        axes[1][0].hist([p.x for p in positions], bins=100)
        axes[1][0].set_xlabel("x")
        axes[1][0].set_ylabel("number of particles")
        axes[1][0].set_title("X Particle Distribution")
        axes[1][1].hist([p.y for p in positions], bins=100)
        axes[1][1].set_xlabel("y")
        axes[1][1].set_ylabel("number of particles")
        axes[1][1].set_title("Y Particle Distribution")
        plt.savefig(f"{images_pathname}{i*h:.4f}.png")
        plt.close()

        if h * i > 10.0:
            ps.unset_potential()

        if passed:
            print(f"\rcalculating step {i + 1}/{n_steps}. Estimated time remaining: {(passed - start_time) * n_steps / (i)}", end="")
        ps.update_yoshida(h)
        passed = time.time()

    return 0


if __name__ == "__main__":
    import sys
    main(sys.argv)
