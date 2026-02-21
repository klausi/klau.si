+++
title = "Python Dbus Gajim Magic"
date = 2008-09-28
path = "python-dbus-gajim-magic"
aliases = ["node/18", "node/18.html", "python-dbus-gajim-magic.html"]
[taxonomies]
tags = ["software", "english"]
+++

<p>I like <a href="http://freedesktop.org/wiki/Software/dbus">Dbus</a> because it allows desktop applications to communicate with each other. I also like <a href="http://www.python.org/">Python</a>, a powerful scripting language that is perfect for short tasks with a minimum of commands. And finally I like <a href="http://www.gajim.org">Gajim</a>, the instant messenger of my choice for the <a href="http://jabber.rwth-aachen.de/wiki/index.php/Jabber_-_Einfach_erkl%C3%A4rt!">XMPP-Jabber network</a> (the only free [free as in free speech] instant messaging network!).</p>
<!-- more -->
<p>Oh I forgot: I really like <a href="http://www.fsf.org">free software</a>! It allows you to study, improve and of course to modify or manipulate the software you use, according to your own needs. Ok, I should stop talking in <a href="http://www.stallman.org/">Richard Stallman</a> style and get to the point. Perhaps non-programmers, non-GNU/Linux-enthusiasts should go away now, it could be boring for you. Find some other sites to waste your time on the internet ;-)</p>
<p>The problem: I want to switch my status message on Gajim very quick, so that people (friends, work colleagues) can see when I'm working or when I'm simply available. I hate it to search and click around in some nested Gajim menus just to toggle my status message. I want to do it with a single click on a button.  The idea: I write a small script, tell Gajim via Dbus to toggle the status message and put a launcher on my GNOME panel bar.</p>
<p><a href="http://xover.fsinf.at/klausi/gajim_switch.py">The code</a>: At first you have to import the dbus library for Python, then you access a remote object from gajim that provides methods to retrieve information or send requests. All this can be done in three lines, perfect for custom scripts to tell applications what they should do. (I would have posted a nicely syntax highlighted code snippet here, but wordpress is not capable of that as far as I can see. I should switch to drupal soon.) It works fine and is really convenient. Feel free to modify the script to your own needs! (I did not add a license to it, because 13 lines aren't worth it - consider it is public domain)</p>
<p>How did I find out: Finding documentation on the internet is hard. In fact, I did a little bit reverse engineering by examining the <a href="http://trac.gajim.org/browser/tags/gajim-0.11.4/src/remote_control.py">source code of gajim for dbus connectivity</a> and an example for a <a href="http://madchicken.altervista.org/tech/2006/10/rhythmbox-096-plugin-for-gajim-status.html">rhythmbox gajim plugin</a>. Well, actually I cannot speak of reverse engineering here, because this is all free and open source software, but digging in python files is ... funny.</p>
        