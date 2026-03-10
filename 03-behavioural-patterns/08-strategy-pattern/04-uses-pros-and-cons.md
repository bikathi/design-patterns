# Uses
- Use the strategy pattern when:
1. You want to use different variations of an algorithm within an object and be able to switch from one algoritm to another
   during runtime.
2. You have a lot of similar classes that only differ in the way they execute some behaviour.
3. You want to isolate the business logic of a class from the implementation details of algorithms that may not be as important
   in the context of that logic.
4. You have massive conditional statement that switches between different variants of the same algorithm.

# Pros
- You can swap algorithms used inside an object at runtime.
- You can isolate the implementation details of an algorithm from the code that uses it.
- You can replace inheritance with composition.
- Open/Closed principle - you can intorude new strategies without having to change the context.

# Cons
- If you only have a couple of algorithms that rarely change, there is no need to overcomplicate the code using this pattern,
  as it comes with many new classes and interfaces that bloat the code.
- Classes must be aware of the differences between strategors to be able to select a proper one.
- A lot of modern programming languages support functional programming that let you implement different versios of an algorithm
  within a set of anonymous functions, then you use the functions exactly as you'd have used the strategy objects, but
  without bloating the code with extra classes and interfaces.
