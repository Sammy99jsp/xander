
def set_seed(seed: int) -> bool:
    """
    Sets the seed for Xander's RNG for this thread to be `seed` if one hasn't already been set.

    Parameters
    ----------
    seed : int
        The seed for this thread's RNG.

    Returns
    -------
    A boolean -- whether this new seed been set.

    Note
    ----
    You can only set the seed once.

    If you do not set a seed with <xander.engine.dice.set_seed>
    or <xander.engine.dice.random_seed>, you will encounter a
    warning on debug builds of Xander.

    Examples
    --------

    Set the seed to 0:

    >>> from xander.engine import dice
    >>> dice.set_seed(0)
    True
    """
    ...

def random_seed() -> bool:
    """
    Sets the RNG seed for this thread to a random value, if one hasn't already been set.

    Returns
    -------
    A boolean -- whether this new seed been set.

    Note
    ----
    You can only set the seed once.

    If you do not set a seed with <xander.engine.dice.set_seed>
    or <xander.engine.dice.random_seed>, you will encounter a
    warning on debug builds of Xander.

    Examples
    --------

    Set the seed randomly:
    
    >>> from xander.engine import dice
    >>> dice.random_seed()
    True
    """
    ...

class Die:
    """An `n`-sided die."""
    def __repr__(self): ...

    def __init__(self, sides: int):
        """
        Create an `n`-sided die.

        Parameters
        ----------
        sides : int
            Number of sides for this die (`n`).
            
        Examples
        --------

        Create and 'roll' a 2-sided die (a coin?!).

        >>> from xander.engine.dice import Die
        >>> coin = Die(2)
        >>> res = coin.roll()
        >>> "Heads!" if res == 2 else "Tails!"
        Heads! 
        """
        pass

    def qty(self, q: int) -> DExpr:
        """
        Make an expression with q copies of this die.
        
        Parameters
        ----------
        q : int
            Number of copies of this die.

        Examples
        --------

        Roll 6d6:

        >>> from xander.engine.dice import D6
        >>> expr = D6.qty(6)
        6d6            
        >>> expr.evaluate()
        (1 + 3 + 1 + 6 + 6 + 2)
        """
        ...

    def advantage(self) -> DExpr:
        ...
    
    def disadvantage(self) -> DExpr:
        ...

    def __add__(self, rhs: Die | DExpr | int) -> DExpr:
        pass

    def roll(self) -> int:
        """Roll this die."""
        pass

D4: Die
"""4-sided Die"""

D6: Die
"""6-sided Die"""

D8: Die
"""8-sided Die"""

D10: Die
"""10-sided Die"""

D12: Die
"""12-sided Die"""

D20: Die
"""20-sided Die"""

D100: Die
"""100-sided Die"""

class DExpr:
    """
    Represents an expression in dice notation, like:
    * `d20 + 6`
    * `adv(d20) + 7`
    """

    def __new__(self, raw: str) -> DExpr:
        """
        Parse dice notation from a string.
        
        Examples
        --------

        Roll a d20 with advantage and disadvantage:

        >>> from xander.engine.dice import DExpr
        >>> adv = DExpr("2d20kh1 + 2d20kl1")
        Adv(d20) + Dis(d20)
        >>> adv.evaluate()
        Adv(--, 20) + Dis(2, --)

        Parse and evaluate "3d10 / 2 + 10":
        
        >>> from xander.engine.dice import DExpr
        >>> expr = DExpr("3d10 / 2 + 10")
        3d10 / 2 + 10
        >>> expr.evaluate()
        (1 + 9 + 2) / 2 + 10
        """
        ...
    
    def __repr__(self): ...

    def __add__(self, rhs: DExpr | Die | int) -> DExpr:
        """
        Adds another dice expression, die, or modifier to this expression.

        Returns:
            A new expression, representing the sum.
        """
        ...

    def advantage(self) -> DExpr:
        """
        Applies advantage to die rolls in this expression.

        Note
        ----
        Advantage will only be applied to a single die rolls in the expression (e.g. `1d20`).

        If you want to apply advantage just to one die in particular,
        use <Die.advantage>.

        Examples
        --------
        
        Giving advantage to an attack roll:

        >>> from xander.engine.dice import D6, D20
        >>> to_hit = (D20 + 5).advantage()
        >>> evaled = to_hit.evaluate()
        >>> print(evaled)
        Adv(--, 20) + 5
        >>> print(evaled.result())
        25
        """
        ...
    
    def disadvantage(self) -> DExpr:
        """
        Applies disadvantage to die rolls in this expression.

        Notes
        -----
        See notes from <DExpr.advantage>

        Examples
        --------

        Give disadvantage to a "to hit" roll:

        >>> from xander.engine.dice import D6, D20
        >>> to_hit = (D20 + 2).disadvantage()
        >>> evaled = to_hit.evaluate()
        >>> print(evaled)
        Dis(2, --) + 2
        >>> print(evaled.result())
        4
        """
        ...

    def evaluate(self) -> DEvalTree:
        """
        Evaluates this expression into a DEvalTree.
        
        Returns:
            A DEvalTree with all dice rolls evaluated, but arithmetic not applied.
            
        Examples
        --------

        Evaluating an attack roll:
        
        >>> from xander.engine.dice import D20
        >>> expr = D20 + 5
        >>> evaled = expr.evaluate()
        >>> print(evaled)
        2 + 5
        """
        ...

class DEvalTree:
    def __repr__(self): ...
    def result(self) -> int:
        ...