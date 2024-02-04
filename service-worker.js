var cacheName = 'yew-pwa';
try {
    var filesToCache = [
        "./ram-webgui",
        "./","./assets/","./assets/github-mark-white.png","./assets/step.svg","./assets/start.svg","./assets/logo_fiit.png","./assets/favicon.ico","./assets/favicon.png","./assets/theme.json","./assets/stop.svg","./ram-webgui-1036f560fa41a015.js","./service-worker.js","./ram-webgui-1036f560fa41a015_bg.wasm","./manifest.json","./loader-f9f833cf1c2e38ab.css","./index.html","./fonts-afe5f0755d102c6d.css","./snippets/","./snippets/ram-webgui-d392777a2c986911/","./snippets/ram-webgui-d392777a2c986911/js/","./snippets/ram-webgui-d392777a2c986911/js/theme.js","./snippets/ram-webgui-d392777a2c986911/js/monarchTokensProvider.js","./snippets/ram-webgui-d392777a2c986911/js/completionItemProvider.js","./snippets/monaco-2292944dc663bfbc/","./snippets/monaco-2292944dc663bfbc/js/","./snippets/monaco-2292944dc663bfbc/js/release/","./snippets/monaco-2292944dc663bfbc/js/release/editor.js","./about-e06e5981598484f0.css","./fonts/","./fonts/Ubuntu-Bold.ttf","./fonts/codicon.ttf","./fonts/Ubuntu-Regular.ttf","./fonts/Ubuntu-Italic.ttf","./fonts/UFL.txt","./fonts/DroidSansMono.ttf","./favicon-3053993e1f74b9c4.ico","./scrollbar-62c6fa4a60935bf8.css","./selection-f0dce743d516a72c.css","./monaco-tweaks-762ec1ba4e6e0956.css","./console-56a61043f2a36de0.css","./style-79fd42931efb1137.css"
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