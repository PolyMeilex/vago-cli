# dirbkm
Quick and easy way to add bookmarks to your favourite directiories
![gif](https://s5.gifyu.com/images/bkm.gif)

# Installation
On Arch linux you can install it from AUR [dirbkm-git](https://aur.archlinux.org/packages/dirbkm-git)

# Usage
```sh
#Bash,Zsh
cd $(dirbkm)
#Fish
cd (dirbkm)
```
Best way to use dirbkm is to add alias to your shell config like so
```sh
#Bash,Zsh
alias cdbkm='cd $(dirbkm)'

#Fish
alias cdbkm "cd (dirbkm)"
```

# Help
```
Linux Directory Bookmarking CLI (dirbkm)

add, -a             Bookmark current dir
add name,-a name    Bookmark current dir with custom name
delete,-d           Delete Bookmark dir
```
