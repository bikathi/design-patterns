**Command Pattern** is also called **Action** or **Transaction** pattern.
It allows you to turn a request into a stand-alone object that contains all information about the request. This transformation
lets you pass requests as method arguments, delay or queue a request's execution and support undoable operations.

# Problem
- Imagine that you're working on a new text editor app. You want to have a toolbar with a bunch of buttons for doing
  various operations in the text editor.
- You decide to create a very neat `Button` class that can render buttons for the toolbar as well as generic buttons e.g. for
  dialog boxes.
- There is now one problem: where will you put the logic that runs when these buttons are clicked?
- One solution would be to create various subclasses e.g. `SaveButton`, `CopyButton`, `CancelButton` e.t.c., each with its own
  logic.
- But this gets messy real quick as you have so many subclasses. Moreso, if you want to change something in the base `Button`
  class, you now have a million other places you might have to change.
