# Rustlings Exercises

Greetings and welcome to `rustlings exercises`. This project contains exercise solutions to get you used to reading and writing Rust code.

## Table of Contents

- [🔎 How to work on this project?](#how-to-work-on-this-project)
- [⚙️ Exercise to Book Chapter mapping](#exercise-to-book-chapter-mapping)
- [🐞 Found a bug or improvement?](#found-a-bug-or-improvement)

## How to work on this project?

The repository and the entire solution module is divided by Git tags. Start from the git tag corresponding to the module you are interested in:

1. Clone the repository, if you haven't already:

```bash
git clone git@github.com:freddydc/rustlings-exercises.git
```

2. Update tag information:

```bash
git fetch --tags
```

3. List the available tags:

```bash
git tag
```

You should see something like:

```bash
1.0.0
```

4. Start a new branch from the point you want:

```bash
git checkout -b name-of-my-branch chosen-label
# For example, to create a branch named 'dev' from '1.0.0' tag
git checkout -b dev 1.0.0
```

🔥 That's all, you can start with all the changes included up to that module.

> 💡 In the [Releases](https://github.com/freddydc/rustlings-exercises/releases) section you can find the entire list of tags.

## Exercise to Book Chapter mapping

| Exercise                                         | Book Chapter |
| ------------------------------------------------ | ------------ |
| [variables](variables)                           | §3.1         |
| [functions](functions)                           | §3.3         |
| [if](if)                                         | §3.5         |
| [move_semantics](move_semantics)                 | §4.1         |
| [primitive_types ](primitive_types)              | §4.3         |
| [structs](structs)                               | §5.1         |
| [enums](enums)                                   | §6           |
| [modules ](modules)                              | §7           |
| [collections ](collections)                      | §8.1, §8.3   |
| [strings ](strings)                              | §8.2         |
| [error_handling ](error_handling)                | §9           |
| [generics ](generics)                            | §10          |
| [option ](option)                                | §10.1        |
| [traits ](traits)                                | §10.2        |
| [tests ](tests)                                  | §11.1        |
| [standard_library_types](standard_library_types) | §13.2        |
| [threads ](threads)                              | §16.1        |
| [macros ](macros)                                | §19.6        |
| [clippy ](clippy)                                | n/a          |
| [conversions ](conversions)                      | n/a          |
| [advanced_errors](advanced_errors)               | n/a          |

## Found a bug or improvement?

Help other students with what you just discovered that would make this course and repository so much better.

- In [Issues](https://github.com/freddydc/rustlings-exercises/issues/new) you can report bugs, add suggestions and comments.
- For its part, the [Pull](https://github.com/freddydc/rustlings-exercises/pulls) Requests will always be open to receive specific improvements.
