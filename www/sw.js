workbox.precaching.precacheAndRoute(self.__precacheManifest);

workbox.routing.registerRoute(
    /\//,
    new workbox.strategies.CacheFirst({
        cacheName: "wasm-sudoku",
        plugins: [
            new workbox.expiration.Plugin({
                maxEntries: 10,
                purgeOnQuotaError: true
            })
        ]
    })
);
