+++
title = "Telling Gajim to remember the Window position"
date = 2007-08-26
path = "telling-gajim-remember-window-position"
aliases = ["node/8", "node/8.html", "telling-gajim-remember-window-position.html"]
[taxonomies]
tags = ["software", "english"]
+++

Recently I deprecated my ICQ account and switched from my old Instant Messaging Client <a href="http://www.pidgin.im" title="Pidign Project Site" target="_blank">Pidgin</a> to <a href="http://www.gajim.org/" title="Gajim Project Site" target="_blank">Gajim</a>, a <a href="http://www.jabber.org/about/overview.shtml" title="Jabber Protocol Overview">Jabber</a>-only IM-client. The good thing: Messages can now be encrypted with <a href="http://en.wikipedia.org/wiki/Pretty_Good_Privacy" title="Wikipedia about PGP" target="_blank">PGP/GPG</a>! The bad thing: The Window position and size was not remembered by Gajim, so the program appeared always on the left instead of the right side, where I placed it, and that on every program start.

<!-- more -->

Solution:
<ol>
	<li>Go to preferences  ==&gt; Advanced ==&gt; Advanced Configuration Editor</li>
	<li>Filter for "roster_x-position", "roster_y-position" and "saveposition"</li>
	<li>Set the values for each to "Activated" (you have to type the String "Activated" in case of "roster_x-posotion" and "roster_y-position")</li>
	<li>Change the position and size of your Gajim buddy list window, restart Gajim and check if the changes are remembered</li>
</ol>
Solution worked with Gajim version 0.11.1.
        