
import numpy as np
from typing import Union, Callable, List
from utils import np_zero_borders


class GridSolver:
    """
    This is a finite-difference integrator that takes n-dimensional data as well as a function describing
    the time derivative and integrates the data numerically, automatically applying default boundary conditions,
    in order to get a time evolution
    """
    def __init__(self, data: np.ndarray):
        """
        :param data: Initial Dataset
        """
        self.data = data
        self.__update_gradients()

    def __update_gradients(self) -> None:
        """
        Update the first and second order gradients of the dataset.
        Boundary conditions are that the first order gradient be zero
        :return: None
        """
        self.gs = np.gradient(self.data)
        # boundary conditions and account for 1-d
        if self.data.ndim == 1:
            self.gs = np_zero_borders(self.gs)
            self.ggs = np.gradient(self.gs)
        else:
            self.gs = [np_zero_borders(g) for g in self.gs]
            self.ggs = [np.gradient(g) for g in self.gs]

    def step(self, f: Callable[[np.ndarray, Union[List, np.ndarray], Union[List, np.ndarray], float], np.ndarray], h: Union[float, int]) -> None:
        """
        Compute one time step on the dataset
        :param f: The time derivative, taking the dataset, its first and second order gradients
                    and a time step as arguments, returning the new values for the dataset
        :param h: The time step
        :return: None
        """
        self.data = f(self.data, self.gs, self.ggs, h)
        self.__update_gradients()

