# Use the adapter pattern when:
1. You want to use some existing class, but its interface isn't compatible with the rest of your code.
2. You want to reuse several existing subclasses but lack some functionality that can't be added to the
  superclass.
  - You start by putting the missing functionality into an adapter class.
  - Then, you wrap objects with missing features inside the adapter, gaining needed features dynamically.
  - For this to work, the target classes must have a common interface, and the adapters field type should
    follow that interface. This approach is also called the **Decorator** patter, which we will learn later.

# Pros
- Single Responsibility principle is adhered to as you separate the data conversion logic from the primary business
  logic of the application.
- Open/Closed principle is adhered to as you can introduce new types of adapters into the program wuthout breaking
  the existing client code, as long as they work with the adapters through the client interface.

# Cons
- Increases teh overall complexity of the code since you have to introduce new interfaces and classes. Sometimes it
  is simpler to just change the service class so that it matches the rest of your code.
