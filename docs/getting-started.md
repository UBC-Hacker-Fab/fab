# Getting Started

## Environment Setup

This repo uses nix for dependency management.

### Install Nix

You may skip this step if you already have nix installed.

#### Linux

##### Ubuntu

```bash
sudo apt install nix
```

##### Other

Install via your systems package manager.

#### MacOS

Nix is not in homebrew so you may install with the following:

```bash
sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install) --daemon
```

### Enable Flakes

If you have flakes already enabled you may skip this step.

Enable flakes by editing `~/.config/nix/nix.conf`

Add the following:

```nix
experimental-features = nix-command flakes
```

> Note if you are on NixOS, edit your config to include
> `nix.settings.experimental-features = [ "nix-command" "flakes" ];` instead.

### (Optional) Install Direnv

Direnv is a tool that allows you to automatically enter the development
environment when in the directory. This is optional but recommended.

### Test Your Installation

If you did not install direnv run the following and check the output:

```bash
nix develop
rustc --version # This should print 1.99.1
picotool version # This should print 2.3.1
```

## Development guidelines

> TODO: Update

Put hardware specific stuff in the `firmware/` workspace

Put platform independent stuff in other crates

Example already showing how unit testing is easy with this pattern.
