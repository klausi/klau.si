+++
title = "Ruby on Rails Setup on Ubuntu 9.10"
date = 2009-12-10
path = "ruby-rails-setup-ubuntu-910"
aliases = ["node/26", "node/26.html", "ruby-rails-setup-ubuntu-910.html"]
[taxonomies]
tags = ["software", "ubuntu", "english"]
+++

<p>I'm working on a Ruby on Rails project right now and had to setup an environment on my Ubuntu 9.10 system. Here are the install commands for your reference if you want to get it done fast.</p>
<!-- more -->
<p>I wanted to have the newest version of the Ruby language (1.9) and Rails plus some extra packages (MySQL support, aspect oriented programming support, debugging support etc.). Some parts needed to be installed from the ubuntu package manager (apt) and some parts were retrieved by the ruby package manager (gem):</p>
<pre>
sudo apt-get install ruby1.9.1
sudo rm /usr/bin/ruby
sudo ln -s ruby1.9.1 /usr/bin/ruby
sudo apt-get install rubygems1.9.1
sudo apt-get install libopenssl-ruby1.9.1
sudo apt-get install ruby1.9.1-dev
sudo apt-get install mysql-server
sudo apt-get install libmysqlclient-dev
sudo apt-get install libsqlite3-dev
sudo gem install rails
sudo gem install sqlite3-ruby
sudo gem install mysql
sudo gem install aquarium
sudo gem install ruby-debug19</pre>
<p>Further steps: there is a very good <a href="http://media.rubyonrails.org/video/rails_blog_2.mov">tutorial screencast available online</a> which shows the power of Rails in 15 minutes.</p>
