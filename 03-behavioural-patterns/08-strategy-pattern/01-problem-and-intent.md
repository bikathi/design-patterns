Is a behavioural design pattern that lets you define a family of algorithms, put each of them into a separate class, and
make their objects interchangeable.

# Problem
- You decide to build a navigation app for casual travelers.
- Your users ask you tom implement a feature that creates automatic route planning i.e. finds the shortest path between
  point A and B depending on the mode of travel.
- You start with rote planning for users using a car.
- You then add an option for walking routes.
- Then you add an option for users using public transport.
- In later code, you plan to support more means of transport, but each time you add a new routing algorithmm the main class of
  the navigator doubles in size, at some point becoming too hard to maintain.
- Any change to the code e.g. adding a new route or a simple bug fix increases the chance of introducing bugs in 
  already-working code.
- So how would we better organize this?