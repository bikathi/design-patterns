# Use the singleton pattern when:
1. A class in you program should just have a single instance available to all clients e.g. a single database object shared
  by different parts of the program.
2. You need stricter control over global variables.

# Pros
- You can be sure that a class only has a single instance
- You gain a global access point to that instance
- The singleton instance is initiated only when it is requested for the first time

# Cons
- Violates the Single Responsibility Principle as the pattern solves two problems at a time
- It can mask bad design, for instance, when the components of the program know too much about each other
- Requires special treatment in a multithreaded environment so that multiple threads won't create a singleton object
several times
- May be hard to unit test, since the constructor of the singleton class is private and overriding static methods is
impossible in most languages, you will need to think of a creative way to mock the singleton.