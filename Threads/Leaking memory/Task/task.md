This task is to implement a multi-threaded `sum` function. 
As guided by the `TODO` comments, leak the input `Vec` using `Vec::leak`, then `sum` its two halves in separate threads.