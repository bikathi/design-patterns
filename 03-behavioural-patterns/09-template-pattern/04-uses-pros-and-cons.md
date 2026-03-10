# Uses
Use the template method when:
1. You want the clients to extend only particular steps of an algorithm but not the whole algorithm of its structure.
2. You have several classes that contain almost identical algorithms with some minor differences. As a result, you might
   need to modify all classes when the algorithm changes.

# Pros
- You can let cleints override only certain parts of a large algorithm, making them less affected by changes that happen
  to other parts of the algorithm.
- You can pull the deuplicate code into a superclass.

# Cons
- Some clients may be limited by the provided skeleton of an algorithm.
- You might violate the *Liskove Substitution Principle* by suppressing a default step implementation via a subclass.
- Template methods tend to be harder to maintain the more steps they have.
