### Explaning the Code - Day 5 of 2024

Day 5 is where the rubber hits the road, literally. This one took a lot of thinking, for me at least.
That said, I felt I should take time to explain the code before, even I, forget what I did.

**Problem statement: process and validate a set of rules and sequences for printing pages.** 

Steps are:

- Creating a map of ordering rules from a given input.

- Checking if specific sequences of pages follow these rules.

- Sorting and reordering the pages based on the rules.

### **Detailed Steps**

~~~
**TLDR**
In simpler terms, the code does the following:

- Create a Rulebook: Reads the rules from the input and creates a book of rules that tells which pages must come after which.

- Checks the Order: For each sequence of pages, checks if they follow the rules in the rulebook.

- Sorts Pages: Reorders any sequences that follow the rules to ensure they are correctly ordered according to the rulebook.~~
~~~

#### 1. Creating the Ordering Map

The `create_ordering_map` function reads the rules from the input data and creates a `hashmap` (dictionary in Python) where each key is a page number and the value is a list of page numbers that must come after the key in the order. 

- Input: The input is a byte array containing page ordering rules followed by sequences of pages. 
The rules are in the form `X|Y`, meaning page X must come before page Y.

- Processing:
  - The function first finds the position in the data where the rules end and the 
  sequences begin `(indicated by \n\n)`. It then splits the rules part and processes each rule line by line.

  - For each rule line (e.g., `47|53`), it splits the line into two parts (47 and 53), converts them into integers, and adds them to the map.
    
  - The resulting `page_ordering_map` is a `hashmap` where keys are page numbers and values are lists of page numbers that must follow the key page.


#### 2. **Checking Order of Pages**
The `is_correctly_ordered `function checks if a given sequence of pages adheres to the ordering rules in the map.

   - Input: A sequence of pages and the ordering map.

   - Processing:

     - The function iterates over each page in the sequence.

     - For each page, it looks up the ordering rules in the map.

     - It then checks if any of the subsequent pages in the sequence violate the ordering rule (i.e., if any page comes after another page it shouldn't).

     - If any rule is violated, the function returns false.

     - If all rules are followed, it returns true.


#### 3. **Reordering Pages**
The `reorder_pages` function attempts to reorder pages sequences that follow the rules and sorts the pages accordingly.

- **Input**: A list of sequences of pages and the ordering map.

- **Processing:**

  - The function iterates over each sequence of pages.

  - For each sequence, it uses the `is_ordered` logic to check if the sequence is correctly ordered.

  - If the sequence is ordered, it sorts the pages according to the rules in the map.

  - The sorted sequence is then added to a list of reordered sequences.

  - The function returns the list of reordered sequences.


