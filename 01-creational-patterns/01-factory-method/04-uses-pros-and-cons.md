# Use the factory method when:
1. You don't know beforehand the exact types and dependencies of the objects your code should work with.
  - For example, to add a new product type to the app, you'll only need to create a new creator subclass and override the
  factory method in it.
  
2. You want to provide users of your library or framework with a way to extend it's internal components
  - For example, imagine that you write an app using an open source UI framework. Your app should have round buttons but
  the framework only provides square ones. You extend the standard *Button* class with glorious *RoundButton* subclass.
  - It works, but now, you need to tell the main *UIFramework* class to use the new button subclass instead of the default one
  - To achieve this, you create a subclass *UIWithRoundButtons* from a base framework class, and override it's *createButton()*
  method. The parent method returned *Button* objects, but you make your override return *RoundButton* objects.
 - Now use the *UIWithRoundButtons* class instead of *UIFramework*.
 - And that's it!

3. You want to save system resources by reusing existing objects instead of rebuilding them each time. 
  - This could be large, resource-intensive tasks such as database connections, file systems and network resources.

# Pros
- You avoid tight coupling between the creator and the concrete products.
- *Single Responsibility Principle* - you can move the product creation code into one place in the program, making the code
easier to support.
- *Open/Close Principle* - you can introduce new types of products into the program without breaking existing client code.

# Cons
- It may become very complicated as a ton of new subclasses are required to implement the pattern.
