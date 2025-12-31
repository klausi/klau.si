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

    placeholder.addEventListener('click', function () {
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
