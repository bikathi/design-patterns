# Use the builder pattern to:
1. Get rid of a 'telescoping constructor'.
  - Say you have a constructor with 10 parameters. Calling such a beast is very inconvenient.
  - You may choose to overload the constructor to have several shaort versions of itself. But now, you have a lot more to
  remember, and all still refer to the main one, meaning you'd still pass default values for ommited parameters.
  - The builder pattern let's you build objects step by step, using only those steps you really need.
  - After implementing the patterns, you don't have to fit dozens of params into your constructors anymore.
  
2. Allow your code to be able to create different representations of the same product (e.g. stone and wooden houses)
  - The builder pattern can be applied when construction of various representations of the product involves similar steps
  that only differ in the details.
  
3. Construct *Composite* (a structural pattern) trees or other complex objects.
  - The builder pattern lets youc onstruct products step-by-step. You could defer execution of some steps without breaking the
  final product.
  - You could even call steps recursively, which comes in handy when you need to build an object tree.
  - A builder doesn't expose the unfinished product while running construction steps. This prevents the client code from
  fetching an incomplete result.
  
# Pros
- You can construct objects step-by-step, defer construction of steps or run steps recursively.
- You can resuse the same construction code when building various representations of products.
- *Single Responsibility* principle - you can isolate comples construction code from the business logic of the product.

# Cons
- The overall complexity of the code increases since the pattern requires creating multiple new classes.