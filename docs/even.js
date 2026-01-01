function initToc() {
  var $toclink = document.querySelectorAll('.toc-link')
  var $headerlink = document.querySelectorAll('.post-content h1 , .post-content h2')
  var $tocLinkLis = document.querySelectorAll('.post-toc-content li')

  var searchActiveTocIndex = function (array, target) {
    if (!array.length) {
      return -1
    }

    target += 30
    for (let i = 0; i < array.length - 1; i++) {
      if (target > array[i].offsetTop && target <= array[i + 1].offsetTop) {
        return i
      }
    }
    if (target > array[array.length - 1].offsetTop) {
      return array.length - 1
    }
    return -1
  }

  document.addEventListener("scroll", function () {
    var scrollTop = document.body.scrollTop | document.documentElement.scrollTop
    var activeTocIndex = searchActiveTocIndex($headerlink, scrollTop)

    $toclink.forEach(function (el) {
      el.classList.remove('active')
    })
    $tocLinkLis.forEach(function (el) {
      el.classList.remove('has-active')
    })

    if ($toclink.length && activeTocIndex !== -1) {
      $toclink[activeTocIndex].classList.add('active')
      let ancestor = $toclink[activeTocIndex].parentNode
      while (ancestor.tagName !== 'NAV') {
        ancestor.classList.add('has-active')
        ancestor = ancestor.parentNode.parentNode
      }
    }
  })
}

function initYoutubeLazy() {
  document.querySelectorAll('.youtube-lazy').forEach(function (container) {
    var placeholder = container.querySelector('.youtube-lazy-placeholder');
    if (!placeholder) {
      return;
    }

    // Add play button dynamically (not in HTML to keep RSS feeds clean)
    var button = document.createElement('button');
    button.className = 'youtube-play-button';
    button.setAttribute('aria-label', 'Play video');
    button.innerHTML = '<svg viewBox="0 0 68 48" width="68" height="48">' +
      '<path class="youtube-play-bg" d="M66.52,7.74c-0.78-2.93-2.49-5.41-5.42-6.19C55.79,.13,34,0,34,0S12.21,.13,6.9,1.55 C3.97,2.33,2.27,4.81,1.48,7.74C0.06,13.05,0,24,0,24s0.06,10.95,1.48,16.26c0.78,2.93,2.49,5.41,5.42,6.19 C12.21,47.87,34,48,34,48s21.79-0.13,27.1-1.55c2.93-0.78,4.64-3.26,5.42-6.19C67.94,34.95,68,24,68,24S67.94,13.05,66.52,7.74z" fill="#f00" />' +
      '<path d="M 45,24 27,14 27,34" fill="#fff" />' +
      '</svg>';
    placeholder.appendChild(button);

    placeholder.addEventListener('click', function (event) {
      event.preventDefault();
      var videoId = container.getAttribute('data-video-id');
      var iframe = document.createElement('iframe');
      iframe.setAttribute('src', 'https://www.youtube-nocookie.com/embed/' + videoId + '?autoplay=1');
      iframe.setAttribute('allow', 'accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture');
      iframe.setAttribute('allowfullscreen', '');
      iframe.setAttribute('title', 'YouTube video player');

      container.innerHTML = '';
      container.appendChild(iframe);
    });
  });
}

if (document.readyState === "complete" ||
  (document.readyState !== "loading" && !document.documentElement.doScroll)
) {
  initToc();
  initYoutubeLazy();
} else {
  document.addEventListener("DOMContentLoaded", initToc);
  document.addEventListener("DOMContentLoaded", initYoutubeLazy);
}
