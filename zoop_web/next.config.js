/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: false,
  // Note: This feature is required to use NextJS Image in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  env: {
    zoopWebsocketServer: "ws://localhost:8080",
    zoopHttpServer: "http://localhost:8080",
    launcherHttpServer: "http://localhost:3000",
  }
}

module.exports = nextConfig

