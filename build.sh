#!/bin/bash

# The docs folder is served by Github pages, so we need this build script to
# output the site there. That way we avoid a Travis build job, which is really
# not necessary.

zola build --output-dir=docs

# Manually copy the old Drupal Planet feed URL because we can't do a HTML
# redirect for XML.
cp docs/tags/drupal-planet/rss.xml docs/categories/drupal-planet/feed
