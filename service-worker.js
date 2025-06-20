const CACHE_NAME = 'sruti-gita-v1';
const urlsToCache = [
  '/',
  '/index.html',
  // You can add paths to your CSS or other static assets here
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg01.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg02.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg03.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg04.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg05.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg06.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg07.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg08.mp3',
  'https://raw.githubusercontent.com/ashutoshmjain/shutri/main/audio/bg09.mp3'
];

self.addEventListener('install', event => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => {
        console.log('Opened cache');
        // It's better to cache audio on demand, but for simplicity, we add them here.
        // For a production app, you might want a more sophisticated caching strategy.
        return cache.addAll(urlsToCache);
      })
  );
});

self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => {
        // Cache hit - return response
        if (response) {
          return response;
        }
        return fetch(event.request);
      }
    )
  );
});

self.addEventListener('activate', event => {
    const cacheWhitelist = [CACHE_NAME];
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (cacheWhitelist.indexOf(cacheName) === -1) {
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

