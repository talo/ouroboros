import json
from typing import Generic, Optional, TypeVar
import uuid

from generic import RuntimeGeneric
from type_info import type_info
    
T = TypeVar('T')

class Object(RuntimeGeneric, Generic[T]):
    def __init__(self, fd: Optional[str] = None):
        self.fd = fd or uuid.uuid4()

    def __lshift__(self, other: T):
        self.assign(other)

    def assign(self, value: T):
        with open(self.fd, 'w') as f:
            json.dump(value, f)

    def get(self) -> T:
        with open(self.fd, 'r') as f:
            return json.load(f)
    
    @classmethod
    def type_info(cls):
        return { "k": "object", "t": type_info(cls.__args__[0]) }