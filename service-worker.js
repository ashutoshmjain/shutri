const CACHE_NAME = 'sruti-gita-v2'; // Changed cache version
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
        console.log('Opened cache and caching initial assets');
        return cache.addAll(urlsToCache);
      })
  );
});

self.addEventListener('activate', event => {
    const cacheWhitelist = [CACHE_NAME];
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (cacheWhitelist.indexOf(cacheName) === -1) {
                        console.log('Deleting old cache:', cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener('fetch', event => {
    event.respondWith(
        caches.open(CACHE_NAME).then(cache => {
            return cache.match(event.request).then(response => {
                // Stale-While-Revalidate Strategy
                const fetchPromise = fetch(event.request).then(networkResponse => {
                    // If we got a valid response, update the cache
                    if (networkResponse) {
                       cache.put(event.request, networkResponse.clone());
                    }
                    return networkResponse;
                });

                // Return the cached response immediately if available,
                // and let the fetch happen in the background.
                return response || fetchPromise;
            });
        })
    );
});

