Memento is also known as snapshot.
Is is a behavioral design pattern that lets you save and restore the previous state of na object without revealing the details
of its implementation.

# Problem
- Imagine your creating a text editor with functionality like text editing, text formatting, insert inline images e.t.c
- At some point you dcide to let users undo any operations carried out on the text - a common and expected feature nowadays.
- You decide to take a direct approach - before performing any operation, the app records the state of all objects into storage.
- Later when the user decides to revert an action, the app fetches the latest snapshot from the history and uses it to restore
  the state of all objects.
- But how do you produce these snapshots? You'd probably have to iterate through all fields of an object and copy their values
  into storage.
- But not all object allow inspection of it's data as most objects declare fields as private.
- Even if all objects marked their fields as public, if you add a new field to any object you must account for it in the
  snapshot logic.
- Even more, the objects being stored as snapshots would most likely need to have all their fields declared as public, to
  allow the retrieval of the data inside them later by the undo logic.
- This makes the exposed data too fragile and they are forced to be this way else we can't reproduce snapshots.