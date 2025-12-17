# Intent
- This is a creational pattern that lets you ensure that a class has only one instance while providing global access to
this instance.

# Problem
- The singleton pattern solves two problems at the same time, violating the Single Responsibility principle:
1. **Ensure a class has a single instance** - e.g. to control access to some shared resource e.g. a database or a file.
2. **Provide a global access point to that instance** - just like a global variable, the Singleton pattern lets you access
some object from anywhere in the program, while protecting the instance from being overwritten by other code.