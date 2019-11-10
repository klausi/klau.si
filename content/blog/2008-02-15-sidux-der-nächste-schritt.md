+++
title = "Sidux - der nächste Schritt"
date = 2008-02-15
path = "sidux-der-nächste-schritt"
aliases = ["node/13", "node/13.html", "sidux-der-nächste-schritt.html"]
[taxonomies]
tags = ["software"]
+++

Lange habe ich auf meinem Desktop-Rechner <a href="http://ubuntu.com" title="Ubuntu Home">Ubuntu GNU/Linux</a> eingesetzt, was für mich zumeist sehr bequem funktioniert hat. Am Anfang wusste ich kaum wie man eine Shell  bedient - es war bei Ubuntu auch nur selten notwendig, vieles ging über grafische Konfigurationstools oder man befragte einfach das ausgezeichnete <a href="http://wiki.ubuntuusers.de/" title="deutsches Ubuntu Wiki">Ubuntuusers-Wiki</a>. Die Jahre zogen ins Land, auf der Uni wurde auch viel mit GNU/Linux/Unix gemacht und ich begann schön langsam die Innereien eines debian-artigen Betriebssystems kennenzulernen (Ubuntu basiert ja auf <a href="http://debian.org" title="Debian Home">Debian</a>). Die Philosophie von freier bzw. open source Software überzeugte mich immer mehr, und ich konnte fast ganz auf Windows XP verzichten (XP habe ich heute noch im <a href="http://wiki.ubuntuusers.de/Dualboot" title="Wikibeitrag zu Dualboot">Dualboot</a>, Vista habe ich gar nie probiert).
<!-- more -->
Dann im Oktober 2007, als Ubuntu 7.10 Gutsy Gibbon erschien, bemerkte ich, dass mein Ubuntu System merklich langsamer geworden war. Das lag meiner Meinung nach an der Service-Inflation: Compiz Fusion, Networkmanager, Power Manager, Tracker, Update Manager, Restricted Drivers Manager, Evolution-Data-Server, etc. etc. Ich fühlte mich von meinem Lieblingsbetriebssystem etwas bevormundet, da auf meinem fix verkabelten Desktop-PC Network und Power Manager mehr als sinnlos waren, und auch die anderen "Spielereien" interessierten mich eigentlich herzlich wenig. Ich wollte (und will noch immer) ein schnelles, voll funktionales System, das meinen Bedürfnissen gerecht wird.

Also installierte ich mir ein minimales Ubuntu ohne grafische Oberfläche, dann den X-Server und schließlich das Paket <em>gnome-core</em>, das eine minimale GNOME Desktopumgebung mitbringt. Ich installierte je nach Bedarf Anwendungen nach und hatte wieder ein halbwegs schnelles System zur Verfügung (der verflixte Power Manager ließ sich aber immer noch nicht abwimmeln). Somit war ich wieder einigermaßen zufrieden, wurde aber trotzdem das Gefühl nicht los, dass vielleicht besser passende Distributionen irgendwo da draußen sein mussten.

Irgendwann probierte ich dann auf einer Test-Partition <a href="http://www.archlinux.de/" title="Arch Linux Home">Arch Linux</a>, das mich aber wegen der fehlenden Vorkonfiguration (z.B. musste die  Konfigurationsdatei xorg.conf selbst editiert werden) eher enttäuschte.

Einige Zeit später kam dann <a href="http://sidux.com" title="Sidux Home">Sidux</a> ins Spiel, eine zum Debian Unstable (Sid) Zweig vollständig kompatible Distribution, die einen grafischen Installer und viele nützliche Skripte mitbringt. Mit zusätzlichen Paketquellen werden neue Kernelversionen und gepatchte (stabilisierte) Software installiert. Vorteile:
<ul>
	<li>Rolling Distribution: Es gibt keine fixen Releases, sondern nur gelgentlich Snapshots vom aktuellen Debian Unstable Zweig auf CD und DVD. Halbjährliche Neuinstallationen oder langwierige Upgrades fallen weg.</li>
	<li>Bleeding Edge: Es ist immer die neuste Software verfügbar. Bei Ubuntu musste ich einige Programme manuell installieren, weil die Versionen in den Paketquellen längst veraltet waren (z.B. pokerth, claws-mail, eclipse, gajim)</li>
	<li>Intelligente Skripte: Mit dem Kommando <em>smxi</em> kann fast das ganze System verwaltet und aktualisiert werden. Es sind keine trägen grafischen Tools oder Hintergrund-Services notwendig.</li>
	<li>Vollständig kompatibel zu Debian: Die Pakete können 1:1 von Debian übernommen und auch entsprechend zurückgegeben werden. Der direkte Support von einer der ältesten und größten Distributionen spricht für sich (Ubuntu kocht ja sein eigenes Süppchen und wird dafür auch regelmäßig kritisiert)</li>
</ul>
Es gibt aber auch Nachteile:
<ul>
	<li>Instabil: Brandaktuelle Software bringt natürlich auch Bugs mit, bis jetzt gab es bei mir aber nur Kleinigkeiten, die mit etwas Erfahrung und einem temporären Downgrade leicht zu beheben waren.</li>
	<li>Vorwissen notwendig: Man sollte schon ein paar grundlegende Dinge über die Paketverwaltung, <em>apt-get</em>, <em>apt-cache</em> und <em>dpkg</em> wissen, um im Ernstfall gerüstet zu sein.</li>
	<li>GNOME wird offiziell nicht unterstützt, läuft bei mir bis jetzt aber tadellos.</li>
</ul>
Ich kann also Sidux für fortgeschrittene und experimentierfreudige Linux-Freunde wärmstens empfehlen.
        