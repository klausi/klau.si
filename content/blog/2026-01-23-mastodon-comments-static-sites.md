+++
title = "Mastodon comments for static sites"
date = 2026-01-23
draft = true
[taxonomies]
tags = ["mastodon"]
+++

I run my blog on the static site generator [Zola](https://www.getzola.org/), but I want to have interactive comments. The solution: Put a post on [Mastodon](https://joinmastodon.org/) and embed the replies under my blog post. This is very simple, low maintenance and independent of bigtech social media - with some limitations in moderation. In this post I will explain the reasoning and how it works.

<!-- more -->

## The advantages of static site generators

I'm really happy about my static blog:

* No maintenance, no security updates. Plain HTML pages generated from Markdown, where I can still make updates very quickly.
* No cost, free hosting on Github Pages (or on Codeberg, Cloudflare, Netlify, Gitlab etc. - you can pick from many options)
* No DDoS attacks from AI crawlers. At work I have to deal with a lot of malicious AI crawlers sending requests from hundreds of thousands of IP addresses to our Drupal sites, causing performance problems. With my static blog I rely on the Github Pages team to deal with that (I only have a few posts and it should not matter anyway).
* Fast page load times - static site generators are perfect to optimize page speed as the server just needs to serve static files. I only have to optimize frontend performance like reducing image file size and hiding those Youtube iframes behind a preview image.
* Green computing: a static site needs tiny server resources compared to dynamic web applications that need to spend compute time for generating web pages. Not saying I'm combating the climate crisis here, but every bit helps.

These advantages are a direct result of 2 fundamental properties of static site generators:

1. Only one person can make changes to the content (me). Site visitors cannot add, change or contribute anything to the static content.
2. Page rendering to HTML is not done each time for every site visitor, but only once during compile time when I deploy changes. This is far more efficient for the use case of a blog for example.

You might already see the gap here...

## Comments on blogs

Interacting with other people can be nice. Sometimes they can add something useful to your content. In the past I did not get a lot of relevant comments. During my blog journey from Wordpress to Drupal to Zola in 2019 I decided to delete all comments and just keep the blog posts. I think nothing much was lost, comments are ephemeral in my opinion. If the comment is really important then people should put it in their own blogpost.

There are also other concerns with comments:

* Moderation: You need to be able to delete bad comments like trolling, harassment or other unwanted [Replyguy behavior](https://reply.hteumeuleu.com)
* Spam: Spammers love blog comments and try to abuse them to advertise their stuff, making money out of your blog post
* Anonymous comments: ideally a comment can be posted by an anonymous user without creating an account

A comment system should be able to address as many of those 3 points as possible. There are different approaches:

### No comments
I was surprised that [Dries Buytaets's Drupal blog](https://dri.es/blog) does not have comments. This is a reasonable choice: do not allow comments in the first place and avoid all the hassle. You are speaking, others need to read and listen - it is your blog afterall. But then I'm asking myself: Why does Dries not use a static site generator if only he publishes content? Probably because there are some other dynamic site elements and it would look bad if the founder of Drupal does not use Drupal for his blog.

### Drupal + comment system
Moderation is easily possible in the backend of the CMS. Antispam and Captcha solutions are available, put need setup and maintenance to really catch spam well. Anonymous comments work out of the box. But now you are maintaining a dynamic web application which is not as simple as a static site generator.

### Static site generator + Javascript loaded comments from a third-party service
The trick here is that the blog content is static and the comments are loaded dynamically with Javascript embed code. Paid services like [Disqus](https://disqus.com/platform/overview/) give you advanced moderation and features like emoji reactions. I don't like it because of cost and being dependent on a third party service that I don't regularly use otherwise. There are other options such as [giscus](https://giscus.app/) or [utterances](https://utteranc.es/) to use Github issues or discussions as comments. But should non-tech folks have to create a Github account to be able to comment on your blog? And many people refuse to use Github because of their contract with the US ICE agency that kills and deports people.

### Static site generator + self-hosted Javascript comments
[Jeff Geerling is doing this: Migrating 13,000 comments from Drupal to Hugo](https://www.jeffgeerling.com/blog/2026/migrating-13000-comments-from-drupal-to-hugo/). He is self-hosting comments in a [Remark42](https://remark42.com/) instance. This is cool, but a non-starter for me. If I would want to maintain a web application then I would stick to Drupal, a system I know very well. It does not make sense for me to learn and run a new system for blog comments. I congratulate Jeff for pulling this off, but I'm too lazy for that. I don't have a community with 13k comments that would be worth it.

### Static site generator + Javascript Mastodon comments
This is currently my preferred solution and very similar to the previous two approaches of using Javascript to load dynamic comments from somewhere. I will go into more details next.

## Mastodon blog comments
[Mastodon](https://joinmastodon.org/) is a federated social network that offers a [public API](https://docs.joinmastodon.org/api/guidelines/) to fetch replies to a post. The idea is that for each blog post there is a corresponding post on Mastodon. People can reply on Mastodon and those replies are treated as comments, fetched with Javascript and displayed under the blog post.

Advantages:
* Visibility on the fediverse: your blog post comments directly exist as replies on the fediverse and are distributed far and wide, allowing quick interaction on the only ethical social media platform I know
* No spam problem - handling spam is out-sourced to the Mastodon instance moderators, which are usually quick.
* Same for heavy harassment: you are not alone in moderating your blog "comments". Mastodon instances have server rules and the most egregious harassment is dealt with quickly. While you are away from the computer it is unlikely that your comment section is brigaded by a harassment campaign. Note that moderation is far from perfect though, see the disadvantages section.
* Built-in notifications: when you use Mastodon anyway you will get notifications when somebody leaves a reply (a "comment")

Disadvantages:
* Anonymous comments are not possible
*
