# Use the memento pattern when:
1. You wan tto produce snapshots of the object's state to be able to restore a previous state of the object.
2. Direct access to an object's fields/getters/setters violates its encapsulation.

# Pros
- You can produce snapshots of the object's state without violating its encapsulation.
- You can simplyfy the originator's code by letting the caretaker maintain the history of the originator's state.

# Cons
- The app might consume lots of RAM if clients create mementos too often.
- Caretakers should track the originator's lifecycle to be able to destroy obsolete mementos.
- Most dynamic programming languages such as PHP, Python and JS can't guarantee that the state within the memento stays
  untouched.
