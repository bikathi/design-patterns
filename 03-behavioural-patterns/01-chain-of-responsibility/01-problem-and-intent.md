**Chain of Responsibility** is a behavioural design pattern that lets you pass requests along a chain of handlers.
Upon receiving a request, each handler decides either to process the request or to pass it to the next handler in the chain.

# Problem
- Imagine that you're working on an online ordering system. You want to restrict access to the system so only authenticated users
  can create orders.
- Also, users who have administrative permissions must have full accesss to all orders.
- After a bit of planning, you realize that these checks must be performed sequentially.
- The application can attempt to authenticate the user to a system whenever it receives a request that contains the user's
  credentials.
- However, if those credentials aren't correct and authentication fails, there is no reason to proceed with any other checks.
- In the next coming months, you decide:
  1. It's unsafe to pass raw dat straight to the ordering system. So you need an extra validation step to sanitize data in the
     request.
  2. You notice the system is vulnerable to brute force password cracking. To negate this, you promptly add a chcek that filters
     repeated failed requests coming from the same IP address.
  3. You could speed up responses by returning cached requests containing the same data. So you add another check which lets the
     request pass through only if there is no suitable cached response.

- All your features and code checks make the code look like a bloated mess which becomes more bloates as you add each new feature
- Changing one feature at times will affect others.
- The system became very hard to comprehend and expensive to maintain, and you struggle with the code a lot.