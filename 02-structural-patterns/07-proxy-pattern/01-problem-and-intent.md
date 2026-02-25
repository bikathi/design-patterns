- Is a structural design pattern that lets you provide a substitute or placeholder for another object.
- A proxy controls access to the original object, allowing you to perform something either befor or after the request
  goes through to the original object.

# Problem
- Imagine you have a massive object that consumes a vast amount of system resources. You need it from time to time, but not
  always.
- We could implement lazy initialization where a client can call some deffered initialization code, when it needs the object.
- But this creates a lot of code duplication.
- Ideally, we would put this object directly inside our object's class, but that isn't always possible, as, for instance, that
  object might be part of a 3rd party library.