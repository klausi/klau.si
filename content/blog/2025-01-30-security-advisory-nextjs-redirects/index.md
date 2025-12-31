+++
title = "Dangerous Next.js redirects - how misconfiguration can bring your website down"
date = 2025-01-30
[taxonomies]
tags = ["security"]
+++

![Black Next.js logos arranged in a circle. In the middle is an emoji with closed eyes exhausting.](nextjs_exhale.webp)

Security Advisory: Next.js Denial of Service vulnerability in redirect misconfiguration

* **Project**: [Next.js](https://nextjs.org/)
* **Security Risk**: Less Critical
* **Vulnerability**: Denial of Service (DoS)
* **Category**: [OWASP A05:2021 – Security Misconfiguration](https://owasp.org/Top10/A05_2021-Security_Misconfiguration/)
* **Affected versions**: all Next.js versions, for example 15.0.3

Note: This vulnerability has been disclosed privately to the Vercel Security Team. They decided that this is a misconfiguration issue and not an inherent security issue.

<!-- more -->

## Description

[Next.js](https://nextjs.org/) is a web framework using React. Configuration for HTTP redirects can be added ([documentation](https://nextjs.org/docs/pages/api-reference/config/next-config-js/redirects)).

The issue happens when misconfiguring a redirect in the pages directory which leads to the Nodejs (Next.js server) process completely hanging, leaving the application subject to denial of service attacks.

Example for a vulnerable configuration: create a page `/other-page`. Then set up a redirect as such:

```js
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,
  async redirects() {
    return [
      {
        source: "/about:path*",
        destination: "/other-page:path*",
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
```

The problem is the `:path` placeholder. There must be a slash `/` before the colon.

## Attack

With the given example an attacker can now send a specially crafted HTTP request to trigger the DoS vulnerability. For example:

```
/about'%20AND%202*3*8=6*8%20AND%20'2tSe'='2tSe/example-se
```

This will send the Node.js process into an endless loop. The Next.js application is not able to handle requests anymore causing the website to be down.

## Mitigation

This vulnerability can mitigated by using slashes correctly in Next.js redirects. The corrected example from above looks like this:

```js
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,
  async redirects() {
    return [
      {
        source: "/about",
        destination: "/other-page",
        permanent: true,
      },
      {
        source: "/about/:path*",
        destination: "/other-page/:path*",
        permanent: true,
      },
    ];
  },
};

export default nextConfig;
```

This has now also been documented as "Good to know" tip on the [Next.js redirects doc page](https://nextjs.org/docs/pages/api-reference/config/next-config-js/redirects): "Remember to include the forward slash `/` before the colon `:` in path parameters of the `source` and `destination` paths, otherwise the path will be treated as a literal string and you run the risk of causing infinite redirects."

There is currently no fix in the Next.js source code to detect and prevent configuration mistakes like this.

## Reported By

This vulnerability was found by Klaus Purer and Joao Garin at [Jobiqo](https://www.jobiqo.com/).

## Conclusion

Web developers can easily make mistakes when setting up redirects in Next.js. It is not clear to me when exactly the redirect vulnerability triggers, as other expressions for the `source` key do not expose this behavior such as `/about-:path(example1|example2)`.

Next.js could be improved to have a redirect loop detection (or infinite loop detection) in the code that handles redirects.
