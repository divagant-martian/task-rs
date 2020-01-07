# Features (or similar)
[ ] Separate library (back-end) from front-end
[ ] Undo in one step
[ ] Report of what I did this week (or between dates)
[ ] V4.0 Better consensus between server instances
[ ] tui front-end
[ ] set a project as a current context in a painless way


# Improvement ideas
[x] Use type aliases for ids and uuids
[x] check if uuids can be something more than Strings
    UUIDS are 128-bit number stored as 16 octets. There is no need to try to do this by hand, there must be a crate for that
[x] Give the task a UUID as a field, not inside the attributes hashmap
[x] Make annotations a Vec<String> (or something) but don't store them in the
    attributes hashmap
[x] If possible, destroy the attributes hashmap completely

# Ideas on doing things differently
- [ ] Projects are not "a field" but a Task. A task can be tagged as a project, a bullet, none of them or something else. This solves the problem on having project hierarchy: A project may have many levels of sub project just by having "subtasks"
- [ ] "Edges" between tasks may be of more than one kind: for instance
  - Blocking/Blocked: A task _can not be started_ without the completion of other tasks
  - Divided in: A task _is considered as completed_ iff all the rest of tasks are completed
  This allows for more complex ideas (which I don't think are relevant but for the sake of extensibility...)
  - a task is considered as completed if one the tasks is completed. (exists => completed) For example "write" is completed if "adding an entry to my journal" or "post on reddit" is completed. You may want to do both, but you can mark "write" as done as soon as one of those is finished.
  - a task is considered as ready/not blocked iff one of the tasks is completed. (exists => not blocked) For example, "work on the paper" can only be started once "organize the desk" or "organize the dining table" is ready because those are the places where you sit to work.
 - [ ] Define a "grammar" (or something) to formalize what is "possible"/"correct"
 - [ ] Use schema.org to model stuff
    - https://schema.org/ActionStatusType
    - https://schema.org/AssignAction
