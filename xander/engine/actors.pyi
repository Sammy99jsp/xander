from xander.engine.combat.action import Action


class Stats:
    @staticmethod
    def from_json(file: str) -> Stats:
        """
        Loads a monster stat block from a JSON stat block file.

        Examples
        --------

        Load a rat:
        >>> from xander.engine.actors import Monster
        >>> rat = Monster.from_json("rat.json")
        >>> rat
        Rat <2/2 HP>
        """
        ...

    def __repr__(self) -> str: ...

    def hp(self) -> int: ...
    
    def max_hp(self) -> int: ...
    
    def temp_hp(self) -> int | None: ...

    def actions(self) -> list[Action]: ...

    @property
    def dead(self) -> bool: ...