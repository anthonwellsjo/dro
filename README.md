     #
  mmm#   m mm   mmm
 #" "#   #"  " #" "#
 #   #   #     #   #
 "#m##   #     "#m#"
###### a super minimal cli todo (dro) application



### why dro?
often lurking in the terminal? just want to keep track of your daily tasks in a minimalist, straight forward manner?

**dro** is designed to be a intuitive and quick way to add notes of things to get done, directly from your favourite command line.


### installation
with homebrew:
```
brew install anthonwellsjo/tap/dro
```

or with cargo:
```
cargo install dro
```

### docs
```
<<<<<<< HEAD
fn show_help(&mut self) {
    self.action_responses.push(ActionResponse {
        message: "
        Command:        Argument:

        s, see          -                   View all dros
        a, add          description         Add new dro with <description>
        md, markdone    index               Mark dro at position <index> as done
        mu, markundone  index               Mark dro at position <index> as undone
        pu, purge       -                   Deletes all dros that are marked as done
        h, help         -                   See documentation
        v, version      -                   See current version
        ",
        _type: ActionResponseType::Content,
        dro: None,
    });
}
```

=======
Command:        Argument:

s, see          -                   View all dros
a, add          description         Add new dro with <description>
md, markdone    index               Mark dro at position <index> as done
mu, markundone  index               Mark dro at position <index> as undone
pu, purge       -                   Deletes all dros that are marked as done
h, help         -                   See documentation
v, version      -                   See current version
```
>>>>>>> dd4a440973ccfc3009406c2a7cdc4d31cb74b92e
