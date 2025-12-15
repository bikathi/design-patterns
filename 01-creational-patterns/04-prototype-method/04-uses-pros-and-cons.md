# Use the prototype pattern when:
1. Your code shouldn't depend on the concrete classes of objects that you need to copy.
  - This happens a lot when your code works with objects passed to you from 3rd party code via some interface. The concrete classes of
  these objects are unknown, and you couldn't depend on them even if you wanted to.
2. You want to reduce the number of subclasses that only differ in the way they initialize their respective objects:
  - Suppose you have a complex class that requires a lot of laborious configuration before it can be used. There are several common ways to
  configure this class, and this code is scattered through your app.
  - To reduce duplication, you create several subclasses and put every common configuration code into their constructors. You've solved the duplication
  problem but now you have lots of dummy subclasses.
  - The prototype pattern lets you use a set of prebuilt objects configured in various ways as prototypes. Instead of instantiating a subclass that
  matches some configuration, the client can simply look for an appropriate prototype and clone it.
  
# Pros
- You can clone objects without coupling their concrete classes.
- You can get rid of repeated initialization code in favor of cloning pre-built prototypes.
- You can produce complex objects more conveniently.
- You get an alternative to inheritance when dealing with configuration presets for complex objects

# Cons
- Cloning complex objects that have circular references can be very tricky.