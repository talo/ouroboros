import json
import sys
from typing import TypeVar, Type
import typing

from mutable import Mutable
from object import Object
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
    
def init(input: type[I], output: type[O]) -> tuple[I, Mutable[O]]:
    if len(sys.argv) < 2:
        sys.stderr.write('error: too few arguments\n')
        sys.exit(1)
    
    if sys.argv[1] == '--introspect':
        data = {
            'name': sys.argv[0],
            'ins': type_info(input),
            'outs': type_info(output),
        }
        json.dump(data, sys.stdout, indent=4)
        print("")
        sys.exit(0)

    if len(sys.argv) < 3:
        sys.stderr.write('error: too few arguments, missing inputs and output\n')
        sys.exit(1)

    return input(json.loads(sys.argv[1])), Mutable(json.loads(sys.argv[2]))

def __test_echo__():
    input, output = init(str, str)
    output << input

def __test_echo_object__():
    input, output = init(list[Object[str]], str)
    output << input.get()

def __test_sum__():
    input, output = init(list[int], int)
    output << sum(input)

def __test_tuple__():
    input, output = init(tuple[list[int], int, Object[tuple[int, int]]], int)
    output << sum(input)

if __name__ == "__main__":
    __test_tuple__()
