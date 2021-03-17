# Rearx
A TUI client for the Searx meta-search engine, written in Rust

# How to use it
Write rearx + "search query" on your terminal.

# Keybinds
Right arrow -> Go to next page\
Left arrow -> Go to previous page\
f -> Go to first page\
q -> Quit\
[0-9] -> opens the search result corresponding to the pressed number key


# Installation
On Arch Linux and derivatives, you can install from the AUR like this: yay -S rearx

On other distros you can install from source by cloning this repo and executing cargo build --release (the only requirement here is to have cargo installed)\
If you installed from source, you need to create the folder ~/.config/rearx/ and put the rearx.yaml configuration file there. Inside this file, you can change the Searx instance you are using.\ 
WARNING: bear in mind that, if the Searx instance you are connecting to has very strict firewall rules regarding json, Rearx will panic! on "rate limit exceeded"

# Platforms
Only Linux is officially supported. Windows support is not planned nor possible because the program depends on the termion crate. It might compile on MacOS, but you are on your own.

# Example screenshot

![alt text](https://github.com/garak92/rearx/blob/3f9b80ce2da33f106a4a1788b510ae9b4064c54c/example.png)
