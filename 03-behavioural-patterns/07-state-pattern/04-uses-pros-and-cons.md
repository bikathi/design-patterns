# Uses
Use the state pattern when:
1. You have an object that behaves differently depending on its current state, the number of states is large, and the
   state-specific code changes frequently.
2. You have a class polluted with massive conditionals that alter how the class behaves according to the current values
   of the class's fields.
3. You have a lot of duplicate code across similar stats and transitrions of a condition-based state machine.

# Pros
- Single Responsibility Principle - organize the code related to particular states into separate classes.
- Open/Closed Principle - introduce new states without changing existing state classes or the context.
- Simplify the code of the context by eliminating bulky state machine conditionals.

# Cons
- Applying the pattern can be overkill if a state machine only has a few states or rarely changes.