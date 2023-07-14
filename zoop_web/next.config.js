/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: false,
  // Note: This feature is required to use NextJS Image in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  env: {
    zoopWebsocketServer: "wss://9241-87-246-163-177.ngrok-free.app",
    zoopHttpServer: "https://9241-87-246-163-177.ngrok-free.app",
    launcherHttpServer: "https://9241-87-246-163-177.ngrok-free.app",
  },
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:8080/:path*' // Proxy to Backend
      }
    ]
  }
}

module.exports = nextConfig

