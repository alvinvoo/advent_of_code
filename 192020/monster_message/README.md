# MonsterMessage

**TODO: Add description**

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `monster_message` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:monster_message, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/monster_message](https://hexdocs.pm/monster_message).

SO NOW, needs to work backward.. since rule 8 and 11 are infinite loops
needs to start FROM the given message list (instead of the rule)
833cb6d commit: finished part 1, answer is 250
0483423 commit: realized that should passin the value instead. another realization is to flatten the list! (remove outer list structure)
                   --> IT WORK! Can print out ["aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb"]
1b87b58 commit: need to call itself for another layer of recursion
37e1c82 commit: realized there are 3 components : | and space, and there's premature splitting at line 29
d98cf11 commit: main recurse_number function; permutate_list function; but still dunno how to join back the list of lists
