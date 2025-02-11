from xander.engine.combat import speed
from xander.engine.combat.action import Action
from xander.engine.combat.action.attack import Attack, AttackResult
from xander.engine.legality import Legality

P3 = tuple[float, float, float]

class Turn:
    def move(self, delta: P3, mode = speed.Walking) -> Legality[None]: ...

    def attack(self, attack: Attack, target: P3) -> Legality[AttackResult]: ...

    def end(self) -> Legality[None]: ...

    def possible_directions(self, mode = speed.Walking) -> Legality[list[tuple[float, float, float]]]: ...

    def __repr__(self) -> str: ...