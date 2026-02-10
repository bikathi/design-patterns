# Use the decorator pattern when:
1. You need to be able to assign exta behaviours to objects at runtime without breaking the code that uses these objects.
2. It is awkwrard or not possible to extend an objects behaviour using inheritance.

# Pros
- You can extend an objects behaviour without making a new subclass.
- You can add or remove responsibilities from an object at runtime.
- You can combine several behaviours by wrapping an object into multiple decorators.
- *Single Responsibility Principle* - you can deivide a monolithic class that implements many possible variants of behaviour
  into several smaller classes.

# Cons
- It is hard to remove a specific wrapper from the wrapper's stack.
- It is hard to implement a decorator in such a way that its behaviour doesn't depend on the order of the decorators in the stack.
- The initial configuration code of layers might look pretty ugly.