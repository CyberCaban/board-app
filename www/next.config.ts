import type { NextConfig } from "next";

// const isProd = process.env.NODE_ENV === "production";
export const api_url =
  process.env.NEXT_PUBLIC_API_BASE_URL || "http://localhost:5000";

const nextConfig: NextConfig = {
  redirects: async () => {
    return [
      {
        source: "/api",
        destination: `${api_url}/api`,
        permanent: false,
      },
    ];
  },
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: `${api_url}/api/:path*`,
        basePath: false,
      },
      {
        source: "/uploads/:path*",
        destination: `${api_url}/uploads/:path*`,
        basePath: false,
      },
      {
        source: "/boards/:path*",
        destination: `${api_url}/boards/:path*`,
        basePath: false,
      },
      {
        source: "/friends/:path*",
        destination: `${api_url}/friends/:path*`,
        basePath: false,
      },
      {
        source: "/chat_source/:path*",
        destination: `${api_url}/chat_source/:path*`,
        basePath: false,
      },
    ];
  },
  images: {
    remotePatterns: [
      {
        hostname: "**",
      },
    ],
  },
};
export default nextConfig;
