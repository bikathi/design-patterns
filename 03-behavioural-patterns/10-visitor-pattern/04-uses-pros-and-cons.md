# Uses
Use the visitor pattern when:
1. You need to perform an operation on all elements of a complex object structure (e.g. an object tree)
2. You need to clean up the business logic of auxiliary behaviours.
3. A behaviour makes sense only in some classes of a class hierarchy but not in others.

# Pros
- Open/Closed principle - you can introduce a new behaviour that can work with objects of different classes without changing
  these classes.
- Single responsibility principle - You can move multiple versions of the same behaviour into the same class.
- A visitor object can accumulate some useful information whicle working with various objects.

# Cons
- You need to update all visitors each time a class gets added or removed from the element hierarchy
- Visitors might lack the necessary access to all the private fields and methods of elements they're supposed to work with.