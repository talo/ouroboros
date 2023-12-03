import json
from typing import Generic, Optional, TypeVar
import uuid

from object import Object
from generic import RuntimeGeneric
from type_info import type_info

T = TypeVar('T')

class Mutable(RuntimeGeneric, Generic[T]):
    def __init__(self, fd: Optional[str] = None):
        self.fd = fd or uuid.uuid4()

    def __lshift__(self, t: T):
        self.assign(t)

    def assign(self, t: T):
        with open(self.fd, 'w') as f:
            json.dump(t, f)
