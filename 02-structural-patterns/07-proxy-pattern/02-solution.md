- The proxy pattern suggests that you create a new proxy class with the same interface as an original service object.
- Then, we update the app to pass around this proxy object to all client's that need the original object.
- When the proxy object receives a request from the client, it creates a real service object and delegates the work to it.
- This way, if we need to execute something before or after the primary logic of the class, the proxy let's us do this
  without changing that class.
- Since the proxy implements the same interface as the original class, it can be passed to any client that expects a real
  service object.
