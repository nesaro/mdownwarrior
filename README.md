# mdownwarrior

A rust clone of taskwarrior that uses markdown as a database.

## Motivation

I used taskwarrior quite a bit in the past, but when the project moved from its custom git friendly storage to sqlite, I lost the ability to synchronise it with git effectively.

Instead I created a set of tools within my notesystem https://codeberg.org/nesaro/mdwt . They work fine and do most of what taskwarrior did, but they are slow and not command line friendly.

So I want to create this repository to emulate the taskwarrior command at a reasonable speed. 

## LLM usage

I am using this projecto to improve my rust skills so it doesn't make sense to use LLMs in any way. However I plan to use my design decisions to write specs for LLM clones to use them.

That way I will be able to prove that I can use LLMs even if they don't provide any benefit. This is the LLM workflow


```ascii
╔═════this═repo══════════╗
║┌───────────┐   ┌─────┐ ║              ┌─────────────────┐
║│programming│   │ spec├─╫─┬─────LLM1──►│ LLMdownwarrior1 │
║│insight    │   └─────┘ ║ │            └─────────────────┘
║└───────────┘           ║ │
╚════════════════════════╝ │            ┌─────────────────┐
                           └─────LLM2──►│ LLMdownwarrior2 │
                                        └─────────────────┘
```
