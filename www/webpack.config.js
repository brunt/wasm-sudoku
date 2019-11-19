const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const WorkboxPlugin = require("workbox-webpack-plugin");
module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin([
            'index.html',
            'styles.css',
            'site.webmanifest',
            'android-chrome-192x192.png',
            'android-chrome-256x256.png',
            'apple-touch-icon.png',
            'favicon.ico',
            'favicon-16x16.png',
            'favicon-32x32.png',
            'safari-pinned-tab.svg'
        ]),
        new WorkboxPlugin.InjectManifest({
            swSrc: "./sw.js",
            swDest: "sw.js"
        })
    ],
};
