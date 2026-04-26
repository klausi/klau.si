#!/bin/bash

# Build the Zola site
rm -rf public
zola build

# Manually copy the old Drupal Planet feed URL because we can't do a HTML
# redirect for XML.
cp public/tags/drupal-planet/rss.xml public/categories/drupal-planet/feed
