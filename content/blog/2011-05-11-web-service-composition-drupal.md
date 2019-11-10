+++
title = "Web Service Composition in Drupal"
date = 2011-05-11
path = "theses"
aliases = ["node/36", "node/36.html", "web-service-composition-drupal.html", "web-service-composition-drupal"]
[taxonomies]
tags = ["software", "uni", "drupal", "drupal planet"]
+++

<p><img alt="" src="/sites/default/files/wsclient_logo_300_0.thumbnail.png" style="width: 209px; height: 217px; float: right;" />A master thesis written by Klaus Purer at the Vienna University of Technology, released in May 2011.</p>
<p><strong>Abstract:</strong></p>
<p>Building web applications has become a complex task and often requires interaction with other web applications, such as web services. Drupal is a free and open source content management system and framework that provides a rich platform for rapid web development. The modular and extensible nature of Drupal allows developers to customize and embrace the core functionality and to create new features. This thesis is about investigating and implementing a web service client module for Drupal that is able to consume classical WS* web services as well as RESTful web services.</p>
<!-- more -->
<p>We will present a web service abstraction model which supports different web service types in order to facilitate integration of web service data into workflows in Drupal. Those workflows are built with the help of a rule engine module (&ldquo;Rules&rdquo;) that offers the creation of event-condition-action rules. We will discuss a solution that provides a web service operation as Rules action and that achieves web service composition by invoking multiple web services in a Rules workflow. This is important for web applications that need to communicate with several external web services and require the orchestration of the data flows between them. Additionally a user interface has been built where web services can be described and used on Drupal administration pages, which means that no programming effort is needed to access web services. Other features such as automatic parsing of WSDL files or sharing of web service descriptions between different Drupal sites are also realized. The implementation has been evaluated and tested on the basis of an automatic translation use-case that is comprised of a workflow with multiple web service invocations.</p>
<p><strong>Resources:</strong></p>
<ul>
	<li>
		The project source code can be found online: <strong><a href="http://drupal.org/project/wsclient">http://drupal.org/project/wsclient</a></strong></li>
	<li>
		The <a href="/sites/default/files/thesis-klausi.pdf">thesis fulltext as PDF</a></li>
	<li>
		The <a href="http://permalink.obvsg.at/AC07810351 ">thesis in the Austrian federal libraries catalog</a></li>
	<li>
		A <a href="/sites/default/files/poster-klausi_0.pdf">poster</a> that summarizes the thesis outcome</li>
	<li>
		Some <a href="/sites/default/files/presentation.pdf">presentation slides</a> (German)</li>
	<li>
		The full <a href="/sites/default/files/klausi_thesis.zip">LaTeX and Scribus sources</a></li>
</ul>

