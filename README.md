```
     #
  mmm#   m mm   mmm
 #" "#   #"  " #" "#
 #   #   #     #   #
 "#m##   #     "#m#"
```
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
Command:         Options:   Argument:
                 
ls, list         -f         -             view all dros
a, add                      description   add new dro with <description>
md, markdone                index         mark dro at position <index> as done
mu, markundone              index         mark dro at position <index> as undone
pu, purge                   -             deletes all dros that are marked as done
h, help                     -             see documentation
v, version                  -             see current version


Options:

-f/-format | d/date | i/index
```
