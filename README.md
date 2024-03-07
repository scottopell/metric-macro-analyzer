# Metric Macro Parser
If a project is using metrics.rs and using the built-in macros to submit
metrics, the goal of this project is to print out the metric names that are
in-use across the project.

Eventually it would be nice to format this in some way or use it to audit that
component comments are up-to-date.

Currently not really working.

```
cargo run -- $HOME/dev/my-project
```
