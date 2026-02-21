+++
title = "Home WiFi setup with Ubuntu, Asus RT-N56U and Linksys WMP600N"
date = 2011-04-15
path = "home-wifi-setup-ubuntu-asus-rt-n56u-and-linksys-wmp600n"
aliases = ["node/35", "node/35.html", "home-wifi-setup-ubuntu-asus-rt-n56u-and-linksys-wmp600n.html"]
[taxonomies]
tags = ["software", "english"]
+++

<p>
	A summary about setting up a 802.11n wireless network at home, also for future self-reference.</p>
<!-- more -->
<p>
	I moved to a new apartment and thought it would be a good idea to avoid network cables and switch to a fast 802.11n wireless network. I bought an Asus RT-N56U wifi router and a Linksys WMP600N PCI card for my desktop PC. Unfortunately there were some problems on the way, so I document the solutions here. All computers involved run Ubuntu 10.10.</p>
<h3>
	Firmware update</h3>
<p>
	The general setup with the Asus router was straight forward and easy, I did it with my notebook and an Ethernet cable. The first mistake was to upgrade the firmware (which is also very easy with this router), because the new firmware version 1.0.1.3 did not work for me (the router could not connect to the modem anymore). Switching back to 1.0.1.2 solved the problem.</p>
<h3>
	DNS problems</h3>
<p>
	Internet was working now, but web pages were somehow delayed and it took several seconds before they loaded. I had a look at <a href="http://speedtest.net/">speedtest.net</a> to examine the ping and bandwidth statistics, but they were totally ok (ping 15ms, bandwidth 20Mbps as promised by my provider). So I started to suspect the DNS settings and fired up wireshark to capture the network traffic. As expected: the DNS responses took several seconds. After some googeling I found the very useful page <a href="http://wiki.ubuntuusers.de/DNS-Probleme">DNS problems (German) at ubuntuusers.de</a> that indicated that my router does not handle DNS requests correctly. I search the DHCP configuration of the router to manually specify DNS servers, but could not find anything.</p>
<p>
	In this case we can configure the Ubuntu network manager to ignore the offered name server(s) from the router and use others. You need to edit your network connection, got to &quot;IPv4 settings&quot; and select &quot;Automatic (DHCP) addresses only&quot;. Then you can enter custom DNS server(s), I used the ones provided by my ISP. And voila: web pages load much faster now.</p>
<h3>
	Wifi card problems</h3>
<p>
	Installing the Linksys WMP600N PCI WiFi card in my desktop PC was not a problem and Ubuntu also detected it correctly. At least I thought so, because there were no WiFi networks showing up in the network manager. After a lot of research about the card&#39;s chipset (Ralink RT2800) I found some errors in the log (&quot;dmesg | grep rt&quot;) while using the rt2800pci linux driver. So I blacklisted that driver (sudo echo &quot;blacklist rt2800pci&quot; &gt;&gt; /etc/modprobe.d/blacklist.conf) and tried the rt2860sta driver that also comes with Ubuntu 10.10. No luck, the WiFi was still dead. The next step was to compile the newest Ralink rt2860sta driver myself (<a href="http://wiki.ubuntuusers.de/WLAN/Ralink#RT28xx">howto on ubuntuusers.de, German</a>). Although everything went through smoothly, still no WiFi.</p>
<p>
	I revisited all hardware components and played around with the card&#39;s antennas - suddenly I was able to receive WiFi signals while scanning (sudo iwlist scan)! Obviously the problem was that the antennas can only pick up a signal when they are aligned in parallel, either strictly vertical or strictly horizontal. I adjusted them to be horizontal because there was no vertical space (cables to the graphics card). Finally everything works now!</p>

        