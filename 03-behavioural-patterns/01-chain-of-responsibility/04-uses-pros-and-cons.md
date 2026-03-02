# Use the CoC pattern when:
1. Your program is expected to process different kinds of requests in various ways but the exact types of requests
   and their sequences are unknown beforehand.
2. It is essential to execute several handlers in a particular order.
3. When the set of handlers and their orders are expected to change at runtime.

# Pros
- You control the order of request handling
- Single responsibilities -  you can decouple classes that invoke operations from classes that perform operations.
- Open/Closed principle - you can introduce new handlers into the app without breaking existing code.