# Use the Bridge pattern when:
1. You want to divide and organize a monolithic class that has several variants of some functionality (for example, if
   the class can work with various different database servers)
2. You need to extend a class in several othagonal (independent) dimensions, like with the *Shapes* and *Colors*
   example in this topic.
3. You need to be able to switch implementations at runtime.

# Pros
- You can create platform independent classes and apps.
- The client code works with high-level abstractions. It isn't exposed to the platform details.
- *Open/Closed principle* as you can introduce new abstractions and implementations independently from each other.
- *Single Responsibility principle* as you can focus on high-level logic in the abstraction and on-platform details in the
  implementation.

# Cons
You make the code more complicated by applying the pattern to a highly cohesive class.