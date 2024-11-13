import type { NextConfig } from "next";

// const isProd = process.env.NODE_ENV === "production";
const api_url = process.env.NEXT_PUBLIC_API_BASE_URL || "http://localhost:5000";

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
