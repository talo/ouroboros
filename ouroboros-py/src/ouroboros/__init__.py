import json
import sys
from typing import Tuple, TypeVar, Type

from mutable import Mutable
from type_info import ouroboros, type_info

@ouroboros
class Bar:
    a: int
    b: str

@ouroboros
class Foo:
    x: int
    y: str
    z: list[int]
    bar: Bar


I = TypeVar('I')
O = TypeVar('O')
    
def init(input_type: Type[I], output_type: Type[O]) -> Tuple[I, Mutable[O]]:
    if len(sys.argv) < 2:
        sys.stderr.write('error: too few arguments\n')
        sys.exit(1)
    
    if sys.argv[1] == '--introspect':
        data = {
            'name': sys.argv[0],
            'ins': type_info(input_type),
            'outs': type_info(output_type),
        }
        json.dump(data, sys.stdout, indent=4)
        sys.exit(0)

    if len(sys.argv) < 3:
        sys.stderr.write('error: too few arguments, missing inputs and output\n')
        sys.exit(1)
    
    pass

if __name__ == "__main__":
    # init(list[int], int)
    # init(tuple[bool], int)
    # init(tuple[bool, int], int)
    # init(tuple[bool, int, float], int)
    # init(tuple[bool, int, float, str], int)
    # init(tuple[bool, int, float, str, tuple[bool]], int)
    init(Foo, int)