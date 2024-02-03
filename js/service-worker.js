var cacheName = 'yew-pwa';
try {
    var filesToCache = [
        "./ram-webgui",
        $FILES_TO_CACHE
    ];
} catch (e) {
    console.error("No files to cache found, something went wrong");
    var filesToCache = [
        "./ram-webgui",
    ];
}

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
    e.waitUntil(
        caches.open(cacheName).then(function (cache) {
            console.log('[ServiceWorker] Caching app shell', filesToCache);
            return cache.addAll(filesToCache);
        })
    );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
    e.respondWith(
        caches.match(e.request).then(function (response) {
            return response || fetch(e.request);
        })
    );
});