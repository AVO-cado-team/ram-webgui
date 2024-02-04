var cacheName = 'yew-pwa';
try {
    var filesToCache = [
        "./ram-webgui",
        "./","./assets/","./assets/github-mark-white.png","./assets/step.svg","./assets/start.svg","./assets/logo_fiit.png","./assets/favicon.ico","./assets/favicon.png","./assets/theme.json","./assets/stop.svg","./service-worker.js","./manifest.json","./loader-f9f833cf1c2e38ab.css","./about-c750dd6c32f74f54.css","./index.html","./fonts-afe5f0755d102c6d.css","./snippets/","./snippets/ram-webgui-962f57e8bf78940b/","./snippets/ram-webgui-962f57e8bf78940b/js/","./snippets/ram-webgui-962f57e8bf78940b/js/theme.js","./snippets/ram-webgui-962f57e8bf78940b/js/monarchTokensProvider.js","./snippets/ram-webgui-962f57e8bf78940b/js/completionItemProvider.js","./snippets/monaco-2292944dc663bfbc/","./snippets/monaco-2292944dc663bfbc/js/","./snippets/monaco-2292944dc663bfbc/js/release/","./snippets/monaco-2292944dc663bfbc/js/release/editor.js","./fonts/","./fonts/Ubuntu-Bold.ttf","./fonts/codicon.ttf","./fonts/Ubuntu-Regular.ttf","./fonts/Ubuntu-Italic.ttf","./fonts/UFL.txt","./fonts/DroidSansMono.ttf","./favicon-3053993e1f74b9c4.ico","./scrollbar-62c6fa4a60935bf8.css","./selection-f0dce743d516a72c.css","./monaco-tweaks-762ec1ba4e6e0956.css","./ram-webgui-a4789f059db7a4e.js","./console-56a61043f2a36de0.css","./ram-webgui-a4789f059db7a4e_bg.wasm","./style-79fd42931efb1137.css"
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