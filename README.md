# SAPM

A System Agnostic Package Manager (SAPM) which provides basic but useful functionality

## Description

SAPM provides all the subcommands necessary for basic system administration. It is can automagically set a default package manager to use depending on the current Distro / OS, or can be configured manually through an environment variable, or as a flag.       

## List Of Supported Package Managers

Package Managers that have a configuration file in /etc/sapm/package_managers/

| Package Managers |
| :--------------: |
|       dnf        |
|      pacman      |

## Future Plans:
(As of 2024-07-07)

In order of decreasing priority:

* Add support for other common Linux package managers: 
  * apt
  * nix
  * portage
  * zypper
  * yum
  * apk
* Add support for Microsoft and macOS package managers:
  * winget
  * homebrew
* Extend functionality of sapm to include other, less common, functionality
  * clean
  * sync
  * purge
