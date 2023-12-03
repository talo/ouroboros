import inspect
import json
from typing import Generic, Optional, TypeVar
import typing
import uuid

from type_info import type_info

class Proxy:
    def __init__(self, generic):
        object.__setattr__(self, '__ouroboros_generic_args__', generic)

    def __getattr__(self, name):
        if typing._is_dunder(name):
            return getattr(self.__ouroboros_generic_args__, name)
        origin = self.__ouroboros_generic_args__.__origin__
        obj = getattr(origin, name)
        if inspect.ismethod(obj) and isinstance(obj.__self__, type):
            return lambda *a, **kw: obj.__func__(self, *a, *kw)
        else:
            return obj

    def __setattr__(self, name, value):
        return setattr(self.__ouroboros_generic_args__, name, value)

    def __call__(self, *args, **kwargs):
        return self.__ouroboros_generic_args__.__call__(*args, **kwargs)

    def __repr__(self):
        return f'<{self.__class__.__name__} of {self.__ouroboros_generic_args__!r}>'

class RuntimeGeneric:
    def __class_getitem__(cls, key):
        generic = super().__class_getitem__(key)
        if getattr(generic, '__origin__', None):
            return Proxy(generic)
        else:
            return generic