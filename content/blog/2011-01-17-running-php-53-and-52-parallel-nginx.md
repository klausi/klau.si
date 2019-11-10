+++
title = "Running PHP 5.3 and 5.2 in parallel with nginx"
date = 2011-01-17
path = "running-php-53-and-52-parallel-nginx"
aliases = ["node/34", "node/34.html", "running-php-53-and-52-parallel-nginx.html"]
[taxonomies]
tags = ["software", "drupal"]
+++

<p>
	The problem: you want to upgrade your Ubuntu server, but you still need PHP 5.2 for some old web sites (i.e. Drupal 5 is not PHP 5.3 compatible).<br />
	The solution: Go with the packaged PHP 5.3 for your new sites and compile PHP 5.2 yourself for the old ones. As we use the nginx webserver with FastCGI, we can choose per site which PHP CGI binary will handle the requests.</p>
<!-- more -->
<p>
	The following steps were carried out on Ubuntu 10.10.</p>
<ul>
	<li>
		Download PHP 5.2 from http://php.net</li>
	<li>
		Extract the archive (tar xvzf php-5.2.X.tar.bz2)</li>
	<li>
		Change to the new directory (cd php-5.2.X)</li>
	<li>
		Run the configure script, we used the following options (&quot;php52&quot; is the name for configuration/data directories, instead of &quot;php5&quot; used by the ubuntu package):</li>
</ul>
<pre>
./configure --prefix=/usr/share/php52\
 --datadir=/usr/share/php52\
 --mandir=/usr/share/man\
 --bindir=/usr/bin/php52\
 --with-libdir=lib64\
 --includedir=/usr/include\
 --sysconfdir=/etc/php52/apache2\
 --with-config-file-path=/etc/php52/cli\
 --with-config-file-scan-dir=/etc/php52/conf.d\
 --localstatedir=/var \
 --disable-debug \
 --with-regex=php \
 --disable-rpath \
 --disable-static \
 --with-pic \
 --with-layout=GNU \
 --with-pear=/usr/share/php \
 --enable-calendar \
 --enable-sysvsem \
 --enable-sysvshm \
 --enable-sysvmsg \
 --enable-bcmath \
 --with-bz2 \
 --enable-ctype \
 --with-db4 \
 --without-gdbm \
 --with-iconv \
 --enable-exif \
 --enable-ftp \
 --with-gettext \
 --enable-mbstring \
 --with-pcre-regex=/usr \
 --enable-shmop \
 --enable-sockets \
 --enable-wddx \
 --with-libxml-dir=/usr \
 --with-zlib \
 --with-kerberos=/usr \
 --with-openssl=/usr \
 --enable-soap \
 --enable-zip \
 --with-mhash=yes \
 --with-exec-dir=/usr/lib/php5/libexec \
 --without-mm \
 --with-curl=shared,/usr \
 --with-zlib-dir=/usr \
 --with-gd=shared,/usr --enable-gd-native-ttf \
 --with-gmp=shared,/usr \
 --with-jpeg-dir=shared,/usr \
 --with-xpm-dir=shared,/usr/X11R6 \
 --with-png-dir=shared,/usr \
 --with-freetype-dir=shared,/usr \
 --with-ttf=shared,/usr \
 --with-t1lib=shared,/usr \
 --with-ldap=shared,/usr \
 --with-ldap-sasl=/usr \
 --with-mysql=shared,/usr \
 --with-mysqli=shared,/usr/bin/mysql_config \
 --with-pspell=shared,/usr \
 --with-unixODBC=shared,/usr \
 --with-xsl=shared,/usr \
 --with-snmp=shared,/usr \
 --with-sqlite=shared,/usr \
 --with-mssql=shared,/usr \
 --with-tidy=shared,/usr \
 --with-xmlrpc=shared \
 --enable-pdo=shared \
 --without-pdo-dblib \
 --with-pdo-mysql=shared,/usr \
 --with-pdo-odbc=shared,unixODBC,/usr \
 --with-pdo-pgsql=shared,/usr/bin/pg_config \
 --with-pdo-sqlite=shared,/usr \
 --with-pdo-dblib=shared,/usr \
 --enable-force-cgi-redirect --enable-fastcgi \	
</pre>
<ul>
	<li>
		The configure script may require additional dev-libraries, install them (incomplete list, you may reuqire others, too):</li>
</ul>
<pre>
aptitude install libxml2-dev libpcre3-dev libbz2-dev libcurl4-openssl-dev libgdbm-dev libdb-dev libjpeg62-dev libpng12-dev libxpm-dev
</pre>
<ul>
	<li>
		Next execute</li>
</ul>
<pre>
make
</pre>
<ul>
	<li>
		Now we can create a package from that and install it:</li>
</ul>
<pre>
sudo checkinstall
</pre>
<ul>
	<li>
		Create the config directory /etc/php52 and copy/link the configuration files from /etc/php5</li>
</ul>
<pre>
sudo mkdir /etc/php52
sudo ln -s /etc/php5/conf.d /etc/php52
sudo ln -s /etc/php5/cli /etc/php52
</pre>
<ul>
	<li>
		Now you can use /usr/bin/php52/php-cgi in your upstart/init script as PHP 5.2 binary. You should also have now a deb-package in the directory where you executed checkinstall. You can use that package on other servers as well (installation: sudo dpkg -i php-5.2.deb).</li>
</ul>

        