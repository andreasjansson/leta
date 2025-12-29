import re
from typing import TypeVar, Callable

T = TypeVar("T")


def validate_email(email: str) -> bool:
    pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    return bool(re.match(pattern, email))


def memoize(func: Callable[..., T]) -> Callable[..., T]:
    cache: dict = {}

    def wrapper(*args, **kwargs):
        key = (args, tuple(sorted(kwargs.items())))
        if key not in cache:
            cache[key] = func(*args, **kwargs)
        return cache[key]

    return wrapper


@memoize
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


class Counter:
    def __init__(self, initial: int = 0):
        self._value = initial

    @property
    def value(self) -> int:
        return self._value

    def increment(self) -> int:
        self._value += 1
        return self._value

    def decrement(self) -> int:
        self._value -= 1
        return self._value

    def reset(self) -> None:
        self._value = 0
