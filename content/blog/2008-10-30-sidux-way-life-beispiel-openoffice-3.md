+++
title = "The sidux way of life - Beispiel OpenOffice 3"
date = 2008-10-30
path = "sidux-way-life-beispiel-openoffice-3"
aliases = ["node/19", "node/19.html", "sidux-way-life-beispiel-openoffice-3.html"]
[taxonomies]
tags = ["software", "german"]
+++

Eine <a href="http://ikhaya.ubuntuusers.de/2008/10/25/ubuntu-rechtfertigt-entscheidung-gegen-openoffice.org-3.0/" target="_blank">Ikhaya Meldung auf ubuntuusers.de</a> samt <a href="http://forum.ubuntuusers.de/topic/ikhaya-ubuntu-rechtfertigt-entscheidung-gegen/">Diskussion</a> brachte mich letztens ein bisschen zum Lächeln: OpenOffice 3 wird nicht in Ubuntu 8.10 Intrepid Ibex enthalten sein. Großes Wehklagen war bei den Fans von Ubuntu (gleichzeitig Fans aktueller Software) zu hören, so was wird mir bei der Distribution meiner Wahl jedenfalls nicht passieren.<!-- more -->

Wie wird OpenOffice 3 in Sidux behandelt? Nun, zum einen gibt es keine fixen Release-Termine bei Sidux, weil das System in kleinen inkrementellen Schritten auf Debian Unstable Basis immer aktuell gehalten wird (Rolling Release). Damit fallen grobe, halbjährliche Upgrades wie bei Ubuntu weg, aktuelle Software kommt ziemlich schnell in den Debian Unstable Zweig und kann unmittelbar verwendet werden. Zum anderen werden mit dem Sidux System- und Upgradewerkzeug <a href="http://manual.sidux.com/de/smxi-de.htm" target="_blank">smxi</a> brandaktuelle Softwarepakete mit Fehlern zurückgehalten, sodass stabiles Arbeiten selbstverständlich ist. OpenOffice 3 ist aber so aktuell, dass es nach dem <a href="http://lists.debian.org/debian-devel-announce/2008/07/msg00007.html" target="_blank">Feature Freeze von Debian</a> im Juli im Moment nicht in Debian Unstable aufgenommen wird.

Kleiner Exkurs zur Wiederholung: Debian hat vier Entwicklungszweige:
<ul>
	<li>Stable (Codename Etch): Aktuelles Debian Relaese, hier werden hauptsächlich Security Updates vorgenommen. Die Software ist teiltweise hoffnungslos veraltet. Gut für StabilitätsfanatikerInnen.</li>
	<li>Testing (Codename Lenny): Nächstes Debian Release, hier werden die Softwarepakete für den zukünftigen Einsatz in Debian 5.0 vorbereitet. Halbwegs aktuell, ziemlich stabil.</li>
	<li>Unstable (Codename Sid): Alles was noch ins nächste Debian Release kommen soll wird einfach paketiert und reingeworfen. Da aber meist nur Software verwendet wird, die auch ein offizielles Release der ursprünglichen EntwicklerInnen (der sogenannte Upstream) darstellt, ist diese oft problemlos verwendbar und sehr aktuell. Was für lauffähig befunden wird, wandert nach einiger Zeit auch weiter nach Testing.</li>
	<li>Experimental: Brandheiß - hier landet neue Software als erstes, oft gibt es nach wenigen Stunden oder Tagen eines Upstream-Releases (z.B. die offizielle Verlautbarung von OpenOffice 3 auf openoffice.org) bereits ein Debian Paket in diesem Zweig. Wird die Software für tauglich befunden, macht sie ihren Weg über Unstable und Testing schließlich irgendwann nach Stable.</li>
</ul>
OpenOffice 3 ist also im Moment in Experimental, da komme ich als Sidux User auch problemlos ran:
<ul>
	<li><code>/etc/apt/sources.list.d/debian.list</code> mit Rootrechten bearbeiten, die Zeile <code>#deb http://ftp.at.debian.org/debian/ experimental main contrib non-free</code> aktivieren oder unten einfügen (dadurch ist eine zweite Quelle aktiv, automatische Upgrades werden aber nur über die erste Quelle, Unstable, durchgeführt)</li>
	<li><code>apt-get update</code></li>
	<li><code>apt-get install -t experimental openoffice.org</code></li>
</ul>
Fertig, jetzt ist OpenOffice 3 installiert und kann verwendet werden. Sollte ich draufkommen, dass OpenOffice 3 noch zu instabil ist, zuviele Bugs hat oder ähnliches, kann ich auch einfach wieder OpenOffice 2.4 aus Unstable installieren:
<ul>
	<li><code>apt-get remove openoffice.org</code></li>
	<li><code>apt-get install openoffice.org</code> (Eine Fehlermeldung tritt auf, das sagt mir, dass ich andere openoffice.org Komponenten auch noch entfernen sollte)</li>
	<li><code>apt-get remove openoffice.org-core openoffice.org-common openoffice.org-l10n-de openoffice.org-base-core</code> (Jetzt werden auch noch alle anderen OpenOffice 3.0 Teile deinstalliert)</li>
	<li>apt-get install openoffice.org</li>
</ul>
Fertig, jetzt habe ich wieder das alte OpenOffice 2.4 installiert und lauffähig (eventuell noch den experimental Eintrag aus der /etc/apt/source.list.d/debian.list entfernen).

Die Vorteile von Sidux dabei aus meiner Sicht:
<ol>
	<li>Keine Fremdquellen in meinen source.list Dateien. Die Pakete die ich installiere kommen von Debian selbst, ich muss mir also keine Gedanken wegen Sicherheitsproblemen machen.</li>
	<li>Schnelle Paketierung: wie gesagt, Debian-Pakete werden sehr schnell nach Upstream-Releases gebaut und stehen in den offiziellen Repositories zur Verfügung (zumindest in experimental).</li>
	<li>Immer auf dem Laufenden - keine halbjährlichen, riesigen Zitter-Upgrades, die spätestens nach dem 3. Mal (also nach 1.5 Jahren) schiefgehen (was mir bei Ubuntu passiert ist).</li>
</ol>
Nachteile gibt es aber auch:
<ol>
	<li>Schwierig für EinsteigerInnen: Du musst schon APT, deine installierten Pakete, smxi etc. etwas  kennen, um nicht überfordert zu sein.</li>
	<li>Update-Flut: Bei Sidux gibt es immer was zu sehen und upzudaten - Leute mit Downloadlimit werden bald ihre Grenzen sprengen.</li>
	<li>Stabil ist relativ - einige Debian-EntwicklerInnen verachten Sidux sogar, weil sie davon ausgehen, dass man ein produktives Betriebssystem gar nicht auf dem Unstable-Zweig betreiben kann. Sidux beweist das Gegenteil, kann aber kurzzeitig Fehler mit sich bringen, die aber sehr schnell behoben werden.</li>
</ol>
        