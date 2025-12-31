+++
title = "Fully hidden automatic system updates on Ubuntu 20.04"
date = 2021-01-05
[taxonomies]
tags = ["ubuntu"]
+++

![Screenshot of Ubuntu's update manager popping up during the movie "Tank Girl."](tank-girl-update-smaller.webp)

Ubuntu's graphical update manager pops up every time you need to install updates. That can be annoying when you are watching a movie or doing other things and don't want to be bothered all the time. Yes, I want to always apply all updates from all sources, but please do it silently. Here is a small script I use to do that with Anacron.

<!-- more -->

Make sure Anacron is installed:

```
sudo apt install anacron
```

Create the file `/etc/cron.daily/autoapt` as root with the following content (inspired by [The Debian Administrator's Handbook](https://debian-handbook.info/browse/stable/sect.automatic-upgrades.html)):

```sh
#!/bin/bash

# This script is useful to automatically update APT packages in the background
# with Anacron.
# See https://klau.si/blog/fully-hidden-automatic-system-updates-ubuntu/

# Print output and log it at the same time.
exec > >(tee -a /var/log/autoapt.log) 2>&1
# We want to see all commands for better debugging in the logs.
set -x
# Log the current date so that we can check when any failed runs happened.
date

export DEBIAN_FRONTEND=noninteractive
apt update
# By default answer all user interaction questions with yes, for example
# for debconf.
# Use the old configuration file when new config files arrive.
# Also say yes to setting up config files.
yes '' | apt \
  -o Dpkg::Options::=--force-confold \
  -o Dpkg::Options::=--force-confdef \
  -y --allow-downgrades --allow-remove-essential \
  --allow-change-held-packages \
  upgrade
# Clean up any packages that are not needed anymore.
apt autoremove -y
# Also update Snap packages. Unfortunately Snap still outputs terminal colors
# - how can we configure snap to not use terminal colors?
snap refresh --color=never --unicode=never
```

This will update all your primary and third-party APT sources and perform any upgrades. Running APT from a script without user interaction is hard, that is why you see all the weird configuration options here.

Make sure this new script file is executable:

```
sudo chmod a+x /etc/cron.daily/autoapt
```

This cron job will run now once per day, whenever you turn on your computer. If anything goes wrong you can check the upgrade log at `/var/log/autoapt.log`.

Sometimes that cron job might not be fast enough and Ubuntu's graphical update manager might still pop up. If you want to get rid of that completely uninstall it:

```
sudo apt remove update-notifier
```

Congratulations, your Ubuntu now updates itself and shuts up about it!
