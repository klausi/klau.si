+++
title = "Stirb Wordpress, stirb!"
date = 2009-02-27
path = "stirb-wordpress-stirb"
aliases = ["node/20", "node/20.html", "stirb-wordpress-stirb.html"]
[taxonomies]
tags = ["software", "german"]
+++

<p>Endlich geschafft: Mein Weblog l&auml;uft jetzt auf <a href="http://drupal.org">Drupal</a> und nicht mehr auf dem unsympathischen <a href="http://wordpress.org">Wordpress</a>. Gleichzeitig bin ich umgezogen - ich will mal den Webspace der TU ausprobieren. Seit einiger Zeit bietet der ZID ja PHP- und Mysql-Unterst&uuml;tzung an, sodass hier auch Content Management Systeme aufgezogen werden k&ouml;nnen.</p>
<!-- more -->
<p>Im Prinzip verlief die &Uuml;bersiedlung problemlos, zuerst habe ich mir die <a href="http://www.zid.tuwien.ac.at/student/internet_services/web/">Informationsseite des ZID</a> angesehen. Datenbank anlegen war kein Problem, Login per SSH auf web.student.tuwien.ac.at auch nicht. Dann das aktuelle Drupal heruntergeladen und in das public_html Verzeichnis entpackt, darin die vorhandene index.html gel&ouml;scht. Im Browser noch die URL meines Webspaces aufgerufen und Drupal mit ein paar Clicks installiert - fertig.</p>
<p>Wie erwartet gab es ein kleines Problem mit den Clean URLs zu l&ouml;sen, da habe ich eine sehr hilfreiche <a href="http://drupal.org/node/121834">Doku-Seite</a> gefunden, die Clean URLs auf Shared Hostern erm&ouml;glicht. Nicht vergessen in der .htaccess-Datei &quot;<code>RewriteBase /</code>&quot; auf &quot;<code>RewriteBase /</code>~eXXXXXXX/&quot; zu &auml;ndern.</p>
<p>Jetzt musste ich noch den gesamten Inhalt meines alten Wordpress-Blogs hierher migrieren, dazu habe ich ein <a href="http://drupal.org/project/wordpress_import">Import-Modul</a> verwendet, das die ganze Arbeit f&uuml;r mich erledigt hat. Leider verlief das nicht ganz fehlerfrei, einige Zeilenumbr&uuml;che in den Posts sind kaputt und m&uuml;ssen wohl oder &uuml;bel manuell gefixt werden.</p>
<p>Zum Abschluss habe ich noch ein paar hilfreiche Module aktiviert/installiert:</p>
<ul>
    <li>Blog - F&uuml;gt Blog Content Type hinzu</li>
    <li><a href="http://drupal.org/project/commentrss">Comment RSS</a> - Damit k&ouml;nnen seitenweit und per Blogeintrag RSS Feeds zu den Kommentaren abonniert werden</li>
    <li><a href="http://drupal.org/project/fckeditor">FCKEditor</a> - WYSIWYG Editor, damit ich beim Erstellen eines Blogposts nicht manuell HTML-Tags einf&uuml;gen muss (z.B. f&uuml;r Links, Aufz&auml;hlungen, etc.)</li>
    <li><a href="http://drupal.org/project/mollom">Mollom</a> - Anti-Spam, damit ihr anonym Kommentare schreiben k&ouml;nnt und ich nicht im M&uuml;ll versinke</li>
</ul>
<p>Jetzt fehlt mir nur noch ein Modul, das automatisch sinnvolle Pfade aus dem Titel eines Blogposts generiert.</p>
<p>Update: <a href="http://drupal.org/project/pathauto">Pathauto</a> gefunden und f&uuml;r besonders wertvoll befunden.</p>
<p>&nbsp;</p>
        