# Use the composite pattern when:
1. You have to implement a tree-like data structure.
2. You want the client code to treat both simple and comples objects uniformly.

# Pros
- You can work with complex tree structures more conveniently: use polymorphism and recursion to your advantage.
- Open/Closed principle: You can introduce new element types into the app without breaking the existing code, which now works
  with the object tree.

# Cons
- It might be difficult to provide a common interface for classes whose functionality differs too much. In certain scenarios,
  youd need to overgeneralize the component interface, making it harder to comprehend.