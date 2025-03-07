# RSLogo Mark Request

## Which stages are complete?

Please mark stages that are complete with an [X].
Stages that are partially complete should be marked with a [?].
Stages that are not complete should be left blank (like [ ]).

- [X] Stage 1 (Pen Controls)
- [X] Stage 2 (Variables + Queries)
- [X] Stage 3 (If + While)
- [X] Stage 4 (The Stack)
- [X] Stage 5 (Procedures)

## Design Excellence

If you have completed a "design excellence" task, please write what
the task was below (note, it MUST be on the list in the assignment or
approved by COMP6991 staff).

> Get 80% test coverage for your application. You could use a library like tarpaulin to calculate your test coverage.

Consider approaching this assignment in a Test-Driven Development style if you're aiming for this design excellence.
This is design excellence because it requires you to think about how to test your program, and to design a testable program.

Please write a paragraph or two (max. 250 words) describing how your design was
influenced by this challenge, with specific reference to sections of your code.

> To achieve 80% test coverage for the application, I adopted a Test-Driven Development (TDD) approach that significantly influenced the design and architecture of the system. This approach required the code to be modular, allowing each component to be tested independently.
> For instance, in the Parser class, methods such as into_tokens() and parse_procedure() were designed to handle specific aspects of the parsing process, making them easier to test in isolation. The into_tokens() method, which converts lines of code into tokens, was specifically crafted to facilitate unit testing by ensuring that it operates purely on input provided via method parameters rather than global state. This separation of concerns ensures that tests can be written to check the method's behavior under various scenarios without side effects from other parts of the application.
> Furthermore, the parse_procedure() method was designed to parse procedures from tokens while maintaining a clear separation from the token generation, which allowed us to mock the input tokens in tests and focus purely on the procedure parsing logic. This method also returns a Boolean indicating success or failure, which aids in creating straightforward, assertive tests that validate whether the procedure parsing was successful given a predefined set of tokens.

## Design Limitations

If you made any design decisions which you now regret making; but don't to go back and change them,
let us know here. You can make up your design marks by acknowledging up to 5 things you would do
differently now.

> I have thought about this design 3 days and then code for only 14 hours, it is a really nice process, nothing too much should be dropped.

## Other Comments

If you want to let us know something else about your program, put it here!
(Feel free to leave this blank)

> Nothing at all
