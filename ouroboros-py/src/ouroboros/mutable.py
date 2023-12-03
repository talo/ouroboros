from typing import Generic, TypeVar

T = TypeVar('T')

class Mutable(Generic[T]):
    def __init__(self):
        pass