+++
title = "Screencasts with RecordMyDesktop, ffmpeg conversion for HTML5 video"
date = 2013-05-12
path = "screencasts-recordmydesktop-ffmpeg-conversion-html5-video"
aliases = ["node/46", "node/46.html", "screencasts-recordmydesktop-ffmpeg-conversion-html5-video.html"]
[taxonomies]
tags = ["software", "ubuntu", "english"]
+++

Producing a demo video for <a href="http://portland2013.drupal.org/session/rest-and-serialization-drupal-8">my DrupalCon Portland presentation</a> caused a bit frustration with video editing tools, so here is a simple solution to cut and convert a video on Ubuntu.<!-- more -->

First I recorded my screencast with RecordMyDesktop, a simple tool to capture the happenings on your screen. It even supports only recording one window, so you don't need to include the rest of your irrelevant desktop. Installation:
```
sudo aptitude install gtk-recordmydesktop
```
The output file it creates uses the Ogg Theroa Video format, which is free and open, but unfortunately not used on a lot on other platforms. For HTML5 video H.264/MPEG-4 now seems to be the most widely spread format, so we need to convert that. I tried with various graphical video editors such as PiTiVi, kdenlive and OpenShot, but not any of them produced useful results. They always changed the resolution, used the wrong codec, a different frame rate or blew up the output file. Their preconfigured rendering profiles just don't make sense for simply converting and cutting a video for the web.

I decided to look into the command line tool ffmpeg directly and found it surprisingly easy to use. Installation with the H.264 codec:
```
sudo aptitude install ffmpeg x264
```
To convert the video file, cut off everything after minute 1:34 and remove the audio track I used this command:
```
ffmpeg -i input-file.ogv -vcodec libx264 -acodec none -t 00:01:34.0 output-file.mp4
```
Done! It also reduced the file size from 4 MB for the Ogg file to 1.4 MB for the MP4 file without reducing the visual quality, thank you ffmpeg :-)
