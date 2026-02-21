+++
title = "Git diff --color-words"
date = 2011-09-25
path = "git-diff-color-words"
aliases = ["node/39", "node/39.html", "git-diff-color-words.html"]
[taxonomies]
tags = ["software", "drupal", "english"]
+++

<p>Git command for better patch review.</p>

<!-- more -->

<p>The usual patch format is often not really human readable. Consider <a href="http://drupal.org/files/issues/dependent-clause-errors-806974-8.patch">this patch on drupal.org</a> (<a href="http://drupal.org/node/806974">issue</a>) that only removes commas. How can I be sure that only the comma was removed on the changed lines? Fortunately, Git can help here. Download the patch, apply it to your repository and use the following command to get a colored output:</p>
<pre>
git diff --color-words</pre>
<p>The output will not show changed lines but changed words, usually removed words in red and added words in green. So you get a much more fine-grained overview of what the patch changes.</p>

        