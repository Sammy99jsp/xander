from typing import ClassVar, overload
from xander.engine.dice import DExpr

class Damage:
    """
    Represents damage (potentially of multiple types)
    """
    def __repr__(self) -> str:
        ...

    def _repr_html_(self) -> str:
        ...

    def __iadd__(self, rhs: Damage):
        ...

    def __add__(self, rhs: Damage) -> Damage:
        ...


class DamageType:
    """
    Represents a type of damage, such as fire, bludgeoning, etc.
    """
    def name(self) -> str:
        ...

    def description(self) -> str:
        ...

    def __repr__(self) -> str:
        ...

    def _repr_html_(self) -> str:
        ...

    @overload
    def __call__(self, amount: DExpr, cause = DamageCause.UNKNOWN) -> Damage:
        """        
        Create damage of this type with the result of rolling <DExpr>.
        
        Parameters
        ----------
        amount: DExpr
            The amount of damage to deal (according to the roll result).
        cause: DamageCause
            The cause of this damage (defaults to <DamageCause.UNKNOWN>).

        Examples
        --------

        Roll 2d6 Fire damage:

        >>> from xander.engine.dice import DExpr
        >>> from xander.engine.damage import Fire
        >>> Fire(DExpr("2d6"))
        (3 + 6) Fire damage
        """
        ...

    @overload
    def __call__(self, expr: str, cause = DamageCause.UNKNOWN) -> Damage:
        """
        Parse the string as a <DExpr>, then create damage
        of this type with the roll result.

        This method otherwise works the same as calling with a DExpr instead.

        Parameters
        ----------
        amount: str
            A string in dice notation representing the amount of damage to deal (according to the roll result).
        cause: DamageCause
            The cause of this damage (defaults to <DamageCause.UNKNOWN>).

        Examples
        --------

        Roll 1d8 Slashing damage:
        >>> from xander.engine.dice import DExpr
        >>> from xander.engine.damage import Slashing
        >>> Slashing("1d8")
        """
        ...
    
    @overload
    def __call__(self, expr: int, cause = DamageCause.UNKNOWN) -> Damage:
        ...

class DamageActor:
    """The entity that dealt the damage."""

    # Static Variables

    UNKNOWN: ClassVar[DamageActor]
    """Damage originating from discernable actor (e.g. the DM)."""

    def __repr__(self) -> str:
        ...

    # TODO: Add a static method to allow from a creature/entity itself.

class DamageSource:
    """The weapon/spell/object that dealt the damage."""

    # Static Variables

    UNKNOWN: ClassVar[DamageSource]
    """No discernable source."""

    def __repr__(self) -> str:
        ...

class DamageCause:
    """The entity that dealt the damage, with the implement that dealt it."""

    # Static Variables

    UNKNOWN: ClassVar[DamageCause]
    """Damage originating from no discernable cause, such as the DM."""

    # Properties.

    actor: DamageActor
    """Who dun' it?"""

    source: DamageSource
    """What they dun' it with?"""

    def __repr__(self) -> str:
        ...


# DAMAGE TYPES

Acid: DamageType
Bludgeoning: DamageType
Cold: DamageType
Fire: DamageType
Force: DamageType
Lightning: DamageType
Necrotic: DamageType
Piercing: DamageType
Poison: DamageType
Psychic: DamageType
Radiant: DamageType
Slashing: DamageType
Thunder: DamageType