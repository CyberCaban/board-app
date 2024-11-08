import type { NextConfig } from "next";

const nextConfig: NextConfig = () => {
  const isProd = process.env.NODE_ENV === "production";
  const api_url = isProd ? process.env.API_BASE_URL : "http://localhost:5000";
  return {
    redirects: async () => {
      return [
        {
          source: "/api",
          destination: api_url,
          permanent: false,
        },
      ];
    },
    rewrites: async () => {
      return [
        {
          source: "/api/:path*",
          destination: `${api_url}/api/:path*`,
        },
        {
          source: "/uploads/:path*",
          destination: `${api_url}/uploads/:path*`,
        }
      ];
    }
  };
};

export default nextConfig;
