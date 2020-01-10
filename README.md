# rust_improvement
Personal rust improvement projects


# Step 1. box_derive_app
This is the test application.  This work starts from the information presented here:
https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5

That blog post is about doing == comparison on disjoint trait objects.  My goal
is to create a "command-queue" that is ordered by time (just a u128). Therefore, I needed to be
able to use comparison operators on disjoint trait objects, rather than ==.


# Step 2. compare_macro
A certain amount of boilerplate will exists on these 'commands'.  This step would be
to replace a lot that with a a macro if possible.



# Conclusion
Goals were reached for the day. Turns out my old comparison code for #1 didn't work quite like
I thought it did so had to redo a couple of things.

Regardless, I did get around to Step 2 and ended up achieving what I wanted. The macro I came
up with is probably nothing extremely novel but it was a good learning exercise and there
might be something in the code worthwhile if you're trying to achieve a macro of your own.


p.s. Look at the tests in "box_derive_app" to see how the command queue behaves.

