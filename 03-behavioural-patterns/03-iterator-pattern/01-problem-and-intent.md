**Iterator** is a behavioural design pattern that lets you traverse elements of a collection without exposing its underlying
representation (list, stack, tree, e.t.c)

# Problem
- Collections are one of the most used data types in programming. It is just a container for a group of objects.
- Most collections store their elements in simple lists, but some others are based on stacks, trees, graphs and other complex
  data structures.
- A constant factor tho is that each data structure must provide a way of accessing its elements so that other code can use
  these elements.
- There should be some way to go through each element of the collection without accessing the same elements over and over.
- For lists this is simple, we use sequential traversal. For more complex stuff like trees, graphs and such, we could be
  constantly switching from depth first search, to breadth first and back and forth.
- The client using these data structures should not know how the data is parsed. So how do we tackle this without tightly
  coupling the client code with the iterators for different data structures.
