# Note CLI

Note CLI is a small program intended for taking note in command line.

## Usage

1. Build the project.
2. Run the program in your terminal.

## Commands

- Help

```shell
~/note_cli help

Note CLI - Take a short note in command line.
Usage:
    list        List of notes
    create      Create new note
    read        Read note
    delete      Delete note
    help        Show this text
```

- Create

```shell
~/note_cli create

Title: test
Content:
wow
```

- List

```shell
~/note_cli list

List of notes:
1: test
```

- Read

```shell
~/note_cli read <index>

test's contents:
wow
```

- Delete

```shell
~/note_cli delete <index>
```
