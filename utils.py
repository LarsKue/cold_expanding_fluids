
import numpy as np
from typing import Iterable, Union, Generator, Sized


def zero_all(a: Union[Iterable, float, int]) -> Generator[Union[Iterable, float, int], None, None]:
    """
    Zero all elements of an n-dimensional Iterable, or just yield zero if the argument is a number
    >>> x = 1; list(zero_all(x))
    [0]
    >>> x = [1, 2, 3]; list(zero_all(x))
    [0, 0, 0]
    >>> x = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]; list(zero_all(x))
    [[0, 0, 0], [0, 0, 0], [0, 0, 0]]

    :param a:
    :return:
    """
    if isinstance(a, Iterable):
        for x in a:
            if isinstance(x, Iterable):
                yield type(x)(zero_all(x))
            else:
                yield 0
    else:
        yield 0


def zero_borders(a: [Iterable, Sized]) -> Generator[Union[Iterable, float, int], None, None]:
    """
    Zero all borders of an n-dimensional array

    1D:
    >>> x = [1, 2, 3]; list(zero_borders(x))
    [0, 2, 0]

    2D:
    >>> x = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]; list(zero_borders(x))
    [[0, 0, 0], [0, 5, 0], [0, 0, 0]]

    3D:
    >>> x = [[[1, 2, 3], [4, 5, 6], [7, 8, 9]], [[10, 11, 12], [13, 14, 15], [16, 17, 18]], [[19, 20, 21], [22, 23, 24], [25, 26, 27]]]; list(zero_borders(x))
    [[[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 14, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]]]

    4D:
    >>> x = np.indices((81,)).reshape((3,3,3,3)).tolist(); list(zero_borders(x))
    [[[[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]]], [[[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 40, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]]], [[[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]], [[0, 0, 0], [0, 0, 0], [0, 0, 0]]]]

    :param a: The array
    :return: None
    """
    if isinstance(a, np.ndarray):
        raise NotImplemented("Use np_zero_borders for numpy arrays.")
    for i, x in enumerate(a):
        if isinstance(x, Iterable):
            if not isinstance(x, Sized):
                raise ValueError("Array values must be numbers or sized iterables.")
            elif i == 0 or i == len(a) - 1:
                yield type(x)(zero_all(x))
            else:
                yield type(x)(zero_borders(x))
        else:
            if i == 0 or i == len(a) - 1:
                yield from zero_all(x)
            else:
                yield x


def np_zero_borders(a: np.ndarray) -> np.ndarray:
    return np.array(list(zero_borders(a.tolist())))


def rel_error(uv, ev):
    """
    Calculate the relative error in an uncertain value to an exact value
    :param uv: uncertain value
    :param ev: exact value
    :return: the relative error between the two
    """
    return abs(uv - ev) / max(abs(uv), abs(ev))


if __name__ == "__main__":
    import doctest
    doctest.testmod()
