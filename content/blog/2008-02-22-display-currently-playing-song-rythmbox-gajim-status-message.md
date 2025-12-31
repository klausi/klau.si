+++
title = "Display the currently playing song from rythmbox as gajim status message"
date = 2008-02-22
path = "display-currently-playing-song-rythmbox-gajim-status-message"
aliases = ["node/14", "node/14.html", "display-currently-playing-song-rythmbox-gajim-status-message.html"]
[taxonomies]
tags = ["software"]
+++

I found a nice feature in the preferences of my instant messenger <a href="http://www.gajim.org" title="Gajim home">gajim</a>, where a check box says "Set status message to reflect currently playing music track". My only problem was that i could not enable the option, because it was greyed out and not active. First I tried to find a plugin for my musicplayer <a href="http://rhythmbox.org" title="Rhythmbox home">r</a><a href="http://rhythmbox.org" title="Rhythmbox home">hythmbox</a>, but the information I found seemed to be old and several web pages said that gajim and rhythmbox should work together natively. I remembered that most <a href="http://gnome.org" title="GNOME home">GNOME</a> applications use <a href="http://www.freedesktop.org/wiki/Software/dbus" title="D-Bus home">D-Bus</a> to communicate with each other. I supposed that there was a D-Bus package missing on my <a href="http://sidux.com" title="Sidux home">Sidux</a> system, so I searched the package repository with "apt-cache search dbus" and found a package named "<a href="http://packages.debian.org/sid/python-dbus" title="Debian package info">python-dbus</a>". Since I know that gajim is written in <a href="http://python.org" title="Python home">python</a> I installed the package and had success: After a restart of gajim the option was now available and worked when enabled.

<!-- more -->

Solution worked with gajim 0.11.4 and rhythmbox 0.10.1.
        