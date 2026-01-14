- You create an *adapter*.
- An adapter is a special object that converts the interface of one object so that another object can understand it.
- The adapater wraps one of the objects to hide the complexity of the conversion happening behind the scene.
- The wrapped object isn't even aware of the adapter.
- For example, you can wrap an object that operates in meters and kilometers with an adapter that converts all of the data
  to imperial units such as feet and miles.
- Adapters are used to make objects with different interfaces collaborate.
- Sometimes, it is even possible to create an adapter that converts calls in both directions.

# Our Problem
- Back to our problem, we can create XML-to-JSON adapters for every class of the analytics library that your code works
  with directly.
- Then, you adjust your code to communicate with the library only via these adapters.
- When an adapter receives a call, it translates the incoming XML data into a JSON structure and passes the call to the
  appropriate methods of a wrapped analytics object.