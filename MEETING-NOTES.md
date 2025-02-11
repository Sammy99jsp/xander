## Meeting Notes

* Serialize from TOML/JSON/whatever :check:
* Combat:
    * Movement:
        * In 5 ft. blocks
    * End Turn
    * Actions:
        * Attack
        * Cast a Spell
        * 

Check out Slay the Spire! (After this, lol).

NN that outputs 2 heads:
* What to do (attack, move, end)
* Targeting ([0; 8])


Actions: [id, [0 x 8]]
State Repr.:
* (10, 5) expressed as ([0 x 9, 1], [0 x 4, 1, 0 x 5])

DoP Video:
* Introduction
* Visualization (clarity > polish)
* Q-Learning Results
* State of the Art:
    * Highlight that this is new!
    * D&D-based RL projects:
        * Lim
* Setup the problem:
    * Why D&D?
    * What can the agent do?
    * Perhaps, some Scenarios:
        * Analyze the strategy
* Future / Next steps?

30/01

* Q-Learning / DQN
* Clean RL
* What does it need to be a MDP?
* What info do you give the agent?

* Describe the attacks of an opponent, and itself.
    * 

* Playing with disadv. range

06/02
* Definition on D&D, mainly  Combat

Past Work
* Andrew Lim's
* Lara Martin's:
    * Proposal for an agent to play D&D
    * 

Why isn't D&D a thing in RL?
* Complexity of the environment
    * D&D is a rule only to be interpreted by humans.
    * It's different to codify for an RL agent to understand
* Ambiguity of the rules:
* What makes a good agent?

Describe the RL problem:
* What players do
* Agent rewards
* Agent actions = { Attack, Move(Left, Up, Down, Right), End }.