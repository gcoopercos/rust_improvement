# rust_improvement
Personal rust improvement projects


# Step 1. box_derive_app
This is the test application.  This work starts from the information presented here:
https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5

That blog post is about doing == comparison on disjoint trait object.  My goal
is to create a "command-queue" that is ordered by time. Therefore, I needed to be
able to use comparison operators on disjoint trait objects, rather than ==.


# Step 2. box_compare_macro
A certain amount of boilerplate will exists on these 'commands'.  This step would be
to replace a lot that with a a macro if possible.


