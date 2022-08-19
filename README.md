# ALPS (Arch Linux Profile Synchronizer) 

ALPS is a minimal backup utility with the purpose of making sharing system configurations easier.

## Usage

**Create a group to get started:**

```
//alps -Ig <group-name>
alps -Ig main
```

Groups act as an instance of a system configuration, allowing you to update your system with their contents.

Using the -I operation, you can add pacman packages, files/configurations, and custom scripts to the group.

**Groups are composed of these three primary pieces:**
- Pacman packages to install to the system 
- Configuration files for those packages
- Scripts to be run following synchronization

```
//add package
alps -Ip main discord rustup fcitx-im 
//add config
alps -Ic main ~/.config/alacritty ~/.config/i3/config /usr/local/bin/coolBinary
//add script
alps -Is main ~/Documents/scripts/sys_temp_checker wifi.sh /usr/bin/nnn 
```

After installing some items to your group, you can sync your system with the group's contents:

```
//packages && configs && scripts of group
alps -Sg main
//packages of group
alps -Sp main
//configs of group
alps -Sc main
//scripts of group
alps -Ss main
```

To share your group with other people, share the folder at ~/.config/alps/<group-name>.

For a full list of commands, ALPS supplies an -h flag for each operation.

## Who is this for?

ALPS can be used with any linux system, although arch will be the only one with integrated package management.

This means any linux user can use ALPS to share configuration files, with the exception of manually specifying package dependencies.

## To do:

- Fully rewrite code now that I am more comfortable with rust
- Integrate more linux package managers
- Add a few QOL flags
- Implement fuzzy finding for config editingImplement fuzzy finding for config editing
