<div align="center">

# `remedy`

[![crates.io](https://img.shields.io/crates/v/remedy.svg)](https://crates.io/crates/remedy)
[![dependency status](https://deps.rs/repo/github/rowanclarke/rs-remedy/status.svg)](https://deps.rs/repo/github/rowanclarke/rs-remedy)
[![build status](https://github.com/rowanclarke/rs-remedy/workflows/CI/badge.svg)](https://github.com/rowanclarke/rs-remedy/actions?workflow=CI)

</div>

## Getting Started

- Install [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
  - Ensure `~/.cargo/bin` is in the `PATH` environment variable.
- Install [`remedy`](https://crates.io/crates/remedy) (`cargo install remedy`)
- Make a workspace directory (`mkdir ~/workspace && cd "$_"`)
- Create a document (`echo "• b89a01bc ·A·Hello· ·B·World·\!" > test.rem`)
- Create a deck from this document (`remedy deck add test.rem`)
- Initialise a session with all decks (`remedy session initialize`)
- Start learning the cards in this session (`remedy session learn`)

### What's going on?

**document**

- A document contains many *rems* - a syntax that can generate cards.
- A rem begins with a bullet `•` followed by an ID of eight hex characters, and contains some text that can optionally be *closed* - this is the text that you want to learn.
- Closed text belongs to a group and is defined by `·<group>·<text>·`.
  - The group may be specified as an ancestor's group by prefixing the group with multiple `^` characters.

**session**

- Implements the simple yet effective [SM2 Algorithm](https://www.supermemo.com/en/blog/application-of-a-computer-to-improve-the-results-obtained-in-working-with-the-supermemo-method).
  - Assess your retention of the card after viewing the answer.
- On quitting the program, the session is saved.
