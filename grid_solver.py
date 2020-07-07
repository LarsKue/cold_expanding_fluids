
import numpy as np
from typing import Union, Callable, List
from utils import np_zero_borders


class GridSolver:
    def __init__(self, data: np.ndarray):
        self.data = data

    def __gradients(self):
        return GridSolver.gradients(self.data)

    @staticmethod
    def gradients(data):
        """
        Update the first and second order gradients of the dataset.
        Boundary conditions are that the first order gradient be zero
        :return: None
        """
        gs = np.gradient(data)
        # boundary conditions and account for 1-d
        if data.ndim == 1:
            gs = np_zero_borders(gs)
            ggs = np.gradient(gs)
        else:
            gs = [np_zero_borders(g) for g in gs]
            ggs = [np.gradient(g) for g in gs]

        return gs, ggs

    def step_euler(self, f: Callable[[Union[float, int], np.ndarray], np.ndarray],
                   t: Union[float, int], h: Union[float, int]) -> None:
        """
        Compute one time step on the dataset using the forward euler method
        :param f: The time derivative function, taking the current time and the current dataset as arguments
        :param t: The current time
        :param h: The time step
        :return: None
        """
        self.data += h * f(t, self.data)

    def solve_euler(self, f: Callable[[Union[float, int], np.ndarray], np.ndarray], n: int, h: Union[float, int], t0=0.0) -> None:
        """
        Compute n time steps of length h using the forward euler method
        See step_euler for details
        :return: None
        """
        for _ in range(n):
            self.step_euler(f, t0, h)
            t0 += h

    def step_rk4(self, f: Callable[[Union[float, int], np.ndarray], np.ndarray], t: Union[float, int], h: Union[float, int]) -> None:
        """
        Compute one time step of length h on the dataset using Runge-Kutta to 4th order
        :param f: The time derivative function, taking the current time and the current dataset as arguments
        :param t: The current time
        :param h: The time step
        :return: None
        """
        k1 = f(t, self.data)
        k2 = f(t + h / 2, self.data + h * k1 / 2)
        k3 = f(t + h / 2, self.data + h * k2 / 2)
        k4 = f(t + h, self.data + h * k3)

        self.data += h * (k1 + 2 * k2 + 2 * k3 + k4) / 6

    def solve_rk4(self, f: Callable[[Union[float, int], np.ndarray], np.ndarray], n: int, h: Union[float, int], t0=0.0):
        """
        Compute n time steps of length h using Runge-Kutta to 4th order
        See step_rk4 for details
        :return: None
        """
        for _ in range(n):
            self.step_rk4(f, t0, h)
            t0 += h
