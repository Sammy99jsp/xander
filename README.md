# 🚧 Xander: A D&D Combat Environment for Reinforcement Learning

Hello, this is a university project that attempts to make an RL environment for D&D 5E combat,
where agents can play the role of monsters against players.

This is still very much in progress.

### Features

Xander supports:
- [x] Dice Notation:
  - [x] `XdY` epxressions
  - [x] Modifiers
  - [x] Advantage/Disadvantage (`2d20kl1`, `2d20kh1`) 
- [x] Stat Blocks:
  - [x] Loading from JSON
  - [x] Ability Scores, Modifiers, and Skills (with proficiencies)
  - [x] HP, Temporary HP
  - [ ] Checks, Saving Throws (in progress)
  - [ ] Death Saves (in progress)
- [ ] Visualization:
  - [ ] Combat arena map (in progress)
  - [ ] `_repr_html_` HTML display for Jupyter (in progress)
- [ ] Reference Agents:
  - [ ] Q-Learning (in progress)

### Implementation
* The core environment is written in Rust, and exposed to python via [pyo3](https://github.com/PyO3/pyo3).
  * Python type bindings are available, but they do have to be manually updated (WIP).
* TODO: write more here.

### Examples
```python
from xander.engine import dice
from xander.engine.actors import Stats

dice.random_seed()

rat = State.from_json("tests/rat.json")
print(rat)
```
