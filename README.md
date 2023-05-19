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
Command:         Flag:      Argument:
                 
ls, list         -f <opt>   -             view all dros
a, add                      description   add new dro with <description>
md, markdone     -i         query/index   mark dro at position <index> as done
mu, markundone   -i         query/index   mark dro at position <index> as undone
pu, purge                   -             deletes all dros that are marked as done
h, help                     -             see documentation
v, version                  -             see current version


Flag options:

-f/-format: d/date, i/index
```

### recommended aliases
```
# DRO
alias d=‘dro’
alias da=‘dro add’
alias dls=‘dro list -f index’
alias dd=‘dro md -i’
alias ddi=‘dro md -i’
```
