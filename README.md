# SAPM

A System Agnostic Package Manager (SAPM) which provides basic but useful functionality

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
