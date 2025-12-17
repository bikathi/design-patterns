- All implementations of the Singleton pattern have two steps in common:
1. Make the default constructor private to prevent other objects from using the `new` operator with the singleton class.
2. Create a static method that acts as a constructor. Under the hood, this method: 
  - calls the private constructor to create an object 
  - saves the object in a static field.
  - all following calls to this method return the cached object.