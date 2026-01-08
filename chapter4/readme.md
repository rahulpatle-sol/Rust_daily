The Stack and the Heap
Many programming languages don’t require you to think about the stack and the heap
very often. But in a systems programming language like Rust, whether a value is on the
stack or the heap affects how the language behaves and why you have to make certain
decisions. Parts of ownership will be described in relation to the stack and the heap later
in this chapter, so here is a brief explanation in preparation.
Both the stack and the heap are parts of memory available to your code to use at
runtime, but they are structured in different ways. The stack stores values in the order it
gets them and removes the values in the opposite order. This is referred to as last in, first
out (LIFO). Think of a stack of plates: When you add more plates, you put them on top of
the pile, and when you need a plate, you take one off the top. Adding or removing plates
from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the
stack, and removing data is called popping off the stack. All data stored on the stack must
have a known, fixed size. Data with an unknown size at compile time or a size that might
change must be stored on the heap instead.
The heap is less organized: When you put data on the heap, you request a certain amount
of space. The memory allocator finds an empty spot in the heap that is big enough, marks
it as being in use, and returns a pointer, which is the address of that location. This process
is called allocating on the heap and is sometimes abbreviated as just allocating (pushing
values onto the stack is not considered allocating). Because the pointer to the heap is a
known, fixed size, you can store the pointer on the stack, but when you want the actual
data, you must follow the pointer. Think of being seated at a restaurant. When you enter,
you state the number of people in your group, and the host finds an empty table that fits
everyone and leads you there. If someone in your group comes late, they can ask where
you’ve been seated to find you.



What Is Ownership?
Ownership is a set of rules that govern how a Rust program manages memory. All programs
have to manage the way they use a computer’s memory while running. Some languages have
garbage collection that regularly looks for no-longer-used memory as the program runs; in
other languages, the programmer must explicitly allocate and free the memory. Rust uses a
third approach: Memory is managed through a system of ownership with a set of rules that the
compiler checks. If any of the rules are violated, the program won’t compile. None of the
features of ownership will slow down your program while it’s running.
Because ownership is a new concept for many programmers, it does take some time to get
used to. The good news is that the more experienced you become with Rust and the rules of
the ownership system, the easier you’ll find it to naturally develop code that is safe and
efficient. Keep at it!
When you understand ownership, you’ll have a solid foundation for understanding the features
that make Rust unique. In this chapter, you’ll learn ownership by working through some