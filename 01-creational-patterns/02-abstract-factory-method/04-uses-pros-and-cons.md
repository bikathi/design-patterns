# Use the abstract factory method when:
1. Your code needs to work with various families of related products, but you don't want it to depend on the concrete classes
of these products - they might be unknown beforehand or you simply want to allow for future extensibility.
  - The Abstract Factory provides you with an interface for creating objects from each class of the product family. As long
  as your code creates objects via this interface, you don't have to worry about creating the wrong variant of a product
  which doesn't match the products already created by your app.
  
2. You have a class with a set of Factory Methods that blur its primary responsibility.
  - In a well-designed program, each class is responsible for only one thing. When a class deals with multiple product types,
  it may be worth extracting its factory methods into a stand-alone factory class or a full-blown Abstract Factory
  implementation.

# Pros
- You can be sure that the products you are getting from a factory are compatible with each other.
- You avoid tight coupling between products and client code.
- *Single Responsibility Principle* - you can extract the product creation code into one place, making the code easier to
support.
- *Open/Closed Principle* - you can interoduce new variants of products without breaking existing client code.

# Cons
- It may become very complicated as a ton of new subclasses are required to implement the pattern.