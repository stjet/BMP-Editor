//https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Offline_Service_workers

self.skipWaiting();

const CURRENT_CACHE = "bmp-editor-07";

self.addEventListener("install", (e) => {
  //console.log("Service worker installed");
  e.waitUntil(
    (async () => {
      //bmp-editor-(hex number increment)
      const cache = await caches.open(CURRENT_CACHE);
      //console.log("Service worker cached");
      await cache.addAll([
        "./",
        "./bmp-editor.wasm",
        "./bmp-editor.js",
      ]);
    })()
  );
});

self.addEventListener('fetch', function(e) {
  //console.log('Service worker fetch', e.request.url);
  e.respondWith(
    caches.match(e.request).then(function(response) {
      return response || fetch(e.request);
    })
  );
});

self.addEventListener("activate", (e) => {
  console.log("activated", e)
  e.waitUntil(
    (async () => {
      let keys = await caches.keys();
      return keys.map((key) => {
        if (key === CURRENT_CACHE) {
          return;
        }
        return caches.delete(key);
      });
    })()
  );
});
