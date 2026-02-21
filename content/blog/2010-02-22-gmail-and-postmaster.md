+++
title = "Gmail and postmaster"
date = 2010-02-22
path = "gmail-and-postmaster"
aliases = ["node/29", "node/29.html", "gmail-and-postmaster.html"]
[taxonomies]
tags = ["software", "english"]
+++

<p>The Gmail spam filter works fine most time, but be careful if you are the postmaster of a mail server and want to check delivery failure notifications in your Gmail account.</p>
<!-- more -->
<p>Postfix forwards the postmaster mails to my Gmail account where all of them were classified as spam. I wondered why I never got any notifications from the mail server I administer and was happy that everything worked out so smoothly. Today I checked the logs and postfix configuration on the server and found out that there were errors I didn't notice.</p>
<p>I checked my Gmail spam folder and found 218 false positives, most coming from the postfix postmaster address of my server, but also from other postmaster addresses of different domains that reported mail delivery problems. I marked them all as &quot;not spam&quot; now and added MAILER-DAEMON@myserver.example.com to the gmail contact list, which will hopefully prevent gmail from classifying the postfix mails as spam.</p>
        