# README

## Requirement Question

> Where might this sort of system be required?

This task scheduler is ideal for managing dependencies in CI/CD pipelines, automated testing, software builds, development workflow automation, and deployment in distributed systems, ensuring tasks execute in correct sequence and conditions.

> What are the categories of closures Rust has?

There are three kinds of closures Rust has:

- Fn: These closures capture variables by reference immutably. They can be called multiple times.
- FnMut: These closures capture variables by mutable reference, allowing them to modify their environment. They can also be called multiple times but require mutable access.
- FnOnce: These closures take ownership of the variables they capture and consume themselves when called, meaning they can be called only once.

> What sort of closure is stored by the scheduler? What does that mean?

`Box<dyn FnMut() -> TaskResult + 'a>` closure is stored by the scheduler:

- Each closure is of the `FnMut` type, which means they capture variables by mutable reference.
- Boxing the closures allows the scheduler to store them uniformly despite their potentially differing sizes and types, given that they all satisfy the FnMut() -> TaskResult trait bound.
- `dyn` means the exact type of the closure isn't known at compile time
- The `'a` lifetime parameter indicates that the closures might reference data that lives as long as the Scheduler.

> What is the lifetime of that closure? Where does it begin and end?

Lifetime parameter is `'a`, the same as struct `Task` and `Scheduler`. The lifetime of the closures begins when they are created and stored in the Scheduler. This occurs when a Task is instantiated and added to the Scheduler's task list using the add_task method. Each Task contains a closure that potentially captures references to variables or resources that must live at least as long as the Task itself is active in the Scheduler.

> What issues does that present?

while using a shared lifetime `'a` ensures type safety and prevents dangling references, it also imposes constraints on how objects can be structured and interact with each other. These constraints can limit design choices and complicate system architecture, particularly in larger or more dynamic applications.

> Can you construct an example program that would be great to work, that doesn't?

```rust
use std::collections::HashSet;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Prerequisites {
    Initialize,
    ProcessData,
    CleanUp,
}

enum TaskResult {
    Finished(HashSet<Prerequisites>),
    RunMeAgain,
}

struct Task<'a> {
    prerequisites: HashSet<Prerequisites>,
    task: Box<dyn FnMut() -> TaskResult + 'a>,
}

struct Scheduler<'a> {
    tasks: Vec<Task<'a>>,
    prerequisites: HashSet<Prerequisites>,
}

impl<'a> Scheduler<'a> {
    fn add_task(&mut self, task: Task<'a>) {
        self.tasks.push(task);
    }

    fn new() -> Self {
        Self {
            tasks: vec![],
            prerequisites: HashSet::new(),
        }
    }
}

fn create_task<'a>() -> Task<'a> {
    Task {
        prerequisites: HashSet::new(),
        task: Box::new(|| {
            println!("Processing data.");
            TaskResult::Finished(HashSet::from([Prerequisites::ProcessData]))
        }),
    }
}

fn main() {
    let mut scheduler1 = Scheduler::new();
    let task = create_task();
    scheduler1.add_task(task);

    let mut scheduler2 = Scheduler::new();
    // This line would ideally reuse the task in another context, but it will fail to compile
    // because `task` lifetime is tied to the first scheduler's scope implicitly.
    scheduler2.add_task(task); // <-- This line will cause a compile-time error
}
```

> How might you change the system to overcome those issues?

Use `clone` or `Rc` to solve ownership model.
