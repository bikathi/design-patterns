# Uses
Use the observer patten when:
1. Changes to the state of one object may require changes to other objects, and the actual set of objects is unknown
   beforehand or changes dynamically.
2. Some objects in your app must observe others but only for a limited time or in specific cases.

# Pros
- Open/Closed principle as you can introduce new subscriber classes without having to change the publisher's code and
  vice versa if there is a publisher interface.
- You can establish relations between objects are runtime.

# Cons
- Subscribers are notified in random order in some cases.