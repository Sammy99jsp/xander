import typing

class Arena:
    """An active arena."""
    def _repr_html_(self) -> str:
        ...

class ProtoArena(typing.Protocol):
    pass

class Simple(ProtoArena):
    """A simple rectangular arena."""
    
    def __init__(self, width: int, height: int):
        ...