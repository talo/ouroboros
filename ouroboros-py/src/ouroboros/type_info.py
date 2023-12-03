from typing import get_type_hints

def __type_info__(t):
    # basics
    if t == bool:
        return "bool"
    if t == int:
        return "i64"
    if t == float:
        return "f64"
    if t == str:
        return "string"
    
    # list
    if t == list or (hasattr(t, "__origin__") and t.__origin__ == list):
        if not hasattr(t, '__args__') or len(t.__args__) != 1:
            raise TypeError("list type must have exactly 1 inner type")
        return {"k": "array", "t": __type_info__(t.__args__[0])}
    
    # tuple
    if t == tuple or (hasattr(t, "__origin__") and t.__origin__ == tuple):
        if not hasattr(t, '__args__') or len(t.__args__) == 0:
            raise TypeError("tuple type must have at least 1 inner type")
        return {"k": "tuple", "t": [__type_info__(x) for x in t.__args__]}
    
    # check for `type_info`` method in `t`` or its super classes
    if hasattr(t, 'type_info') and callable(getattr(t, 'type_info')):
        return t.type_info()
    
    # error
    raise TypeError(f"unsupported type {t}")

def ouroboros(cls):
    def type_info():
        hints = get_type_hints(cls)
        info = {"k": "record", "t": {}}
        for attr, typ in hints.items():
            info["t"][attr] = __type_info__(typ)
        return info

    setattr(cls, 'type_info', staticmethod(type_info))
    return cls

def type_info(t):
    return __type_info__(t)