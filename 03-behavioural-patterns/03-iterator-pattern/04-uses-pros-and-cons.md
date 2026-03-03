# Use the Iterator pattern when:
1. Your collection has a complex data structure under the hood, but you want to hide its complexity from client (either for
   security or convenience reasons)
2. You want to reduce the duplication of traversal code across your appl
3. You want your code to be able to traverse different data structures or when types of these structures are unknown beforehand.

# Pros
- Single Responsibility Principle as you clean up the client code and the collections by extracting bulky traversal algorithms
  into separate classes.
- Open/Closed principe - you can implement new types of collections and iterators and pass them to existing code without
  breaking anything.
- You can iterate over the same collection in parallel because each iteraror object contains its own iteration state.
- You can also delay an iteration and continue it when needed.

# Cons
- Applying the pattern is an overkill if your app only works with simple collections.
- Using an iterator may be less efficient than going through elements of some specialized collections directly.