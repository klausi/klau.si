+++
title = "Installation of a Drupal Upgrade Path Test DB"
date = 2012-03-31
path = "installation-drupal-upgrade-path-test-db"
aliases = ["node/41", "node/41.html", "installation-drupal-upgrade-path-test-db.html"]
[taxonomies]
tags = ["drupal", "drupal planet", "english"]
+++

<p>While working on the <a href="http://drupal.org/node/935062">Kill role IDs patch</a> I had to develop some upgrade path Simpletests. Unfortunately the <a href="http://drupal.org/node/1429136">documentation for upgrade path tests</a> currently lacks installtion instructions on how to work with the existing bare and filled exported test databases. Here is a small writeup of what I did until we document that properly.</p>
<!-- more -->
<p>The test databases are exported to executable PHP code and live in core/modules/simpletest/tests/upgrade (Drupal 8). The problem is that you needed a bootstrapped Drupal database system to run those statements and an empty database at the same time where you insert the data. So first you need to create a new, empty development environment - not only a new site in a multi-site setup in your usual development directory.</p>
<ul>
	<li>
		Checkout Drupal core (7.x in my case) in a separate directory, create a new database, setup your web server (this empty instance is called d7upgrade.localhost on my machine).</li>
	<li>
		You will also need an existing Drupal 7/8 installation, you can just use any development site you already have (this instance is called drupal-8.localhost on my machine).</li>
	<li>
		Extract the zipped drupal-7.bare.database.php.gz or drupal-7.filled.database.php.gz in drupal-8/core/modules/simpletest/tests/upgrade: <code>gunzip drupal-7.bare.database.php.gz</code></li>
	<li>
		Add database credentials for the d7upgrade database to settings.php of drupal-8.localhost: <?php
$databases['d7upgrade']['default'] = array (
  'database' => 'd7upgrade',
  'username' => 'root',
  'password' => '',
  'host' => 'localhost',
  'port' => '',
  'driver' => 'mysql',
  'prefix' => '',
);
?></li>
	<li>
		Create a small PHP script called import.php somewhere in drupal-8 that will insert the data to d7upgrade: <?php
db_set_active('d7upgrade');
require '/home/klausi/workspace/drupal-8/core/modules/simpletest/tests/upgrade/drupal-7.bare.database.php';
?></li>
	<li>
		Execute the script with drush somewhere in drupal-8: <code>drush php-script import.php</code></li>
	<li>
		Now the d7upgrade database should be filled. Go to d7upgrade.localhost in your browser and install Drupal, after entering the database credentials you should get to an existing site.</li>
	<li>
		Login as admin with user: drupal and password: drupal. Make changes to the installation that you need for your upgrade test (e.g. I added some user roles that I wanted to do tests with).</li>
	<li>
		Export the modified database (you need to run that from the root of your d7upgrade installation): <code>php scripts/dump-database-d7.sh > d7upgrade.php</code></li>
	<li>
		Examine the differences of your export to the original export (note: you will see quite some unrelated changes as the original export may not be fully up to date): <code>diff d7upgrade.php /home/klausi/workspace/drupal-8/core/modules/simpletest/tests/upgrade/drupal-7.bare.database.php | less</code></li>
	<li>
		Copy your modifications to a separate PHP file, as for example done in drupal-7-language.php. By doing that you avoid to have another big full database export and you can reuse drupal-7.bare.database.php.gz or drupal-7.filled.database.php.gz in your UpgradePathTestCase::setUp() (see upgrade.language.test for an example).</li>
</ul>
<p>Doing all this is ugly and I&#39;m not sure this is the way it should be done. Leave a comment or contact me if you know better.</p>

        