+++
title = "Drupal Core update script"
date = 2009-12-22
path = "drupal-core-update-script"
aliases = ["node/27", "node/27.html", "drupal-core-update-script.html"]
[taxonomies]
tags = ["software", "drupal"]
+++

<p>
	<strong>Update: Do not use this script anymore, drush provides a very good upgrade mechanism itself now!</strong></p>
<p>
	Upgrading <a href="http://drupal.org">Drupal</a> core can be a very tedious task - especially if you run a Drupal multi site installation (backup database, put site into maintenance mode, run update DB script, put site online again - repeat for every single site). We use <a href="http://drupal.org/project/drush">Drush</a> and a custom bash script to make life much more easier on our fsinf.at Drupal farm.</p>
<!-- more -->
<p>
	For those inpatient people who do not want to read on: you can find the <a href="http://websvn.fsinf.at/filedetails.php?repname=klausi&amp;path=%2Fdrupal_upgrade_farm.sh">script in our subversion repository.</a></p>
<p>
	Requirements for running the script:</p>
<ul>
	<li>
		Do not use the script unmodified! It contains several settings that are tied to the fsinf.at setup, so please look through the code and customize it to you needs.</li>
	<li>
		We use a symbolic link &quot;drupal&quot; that points to the drupal directory &quot;drupal-6.15&quot;. Drush uses per default the same naming convention, so the future Drupal 6.16 will be downloaded to &quot;drupal-6.16&quot;. We copy over settings and sites to that directory and then redirect the symbolic link to the new directory.</li>
	<li>
		Currently the script is designed to be executed as root user.</li>
</ul>

        