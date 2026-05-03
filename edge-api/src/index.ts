export interface Env {
  APP_NAME: string;
  COURSE_NAME: string;
  API_TOKEN: string;
  ADMIN_EMAIL: string;
  SETTINGS: KVNamespace;
}

function jsonResponse(
  data: Record<string, unknown>,
  init: ResponseInit = {},
): Response {
  return Response.json(data, {
    headers: { "cache-control": "no-store", ...init.headers },
    ...init,
  });
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    console.log("request", {
      path: url.pathname,
      method: request.method,
      colo: request.cf?.colo,
      country: request.cf?.country,
    });

    switch (request.method) {
      case "GET": {
        switch (url.pathname) {
          case "/": {
            return jsonResponse({
              app: env.APP_NAME,
              course: env.COURSE_NAME,
              message: "Hello from Cloudflare Workers",
              timestamp: new Date().toISOString(),
              endpoints: [
                "/",
                "/health",
                "/edge",
                "/config",
                "/counter",
                "/counter/reset",
              ],
            });
          }
          case "/health": {
            return jsonResponse({
              status: "ok",
              service: env.APP_NAME,
              timestamp: new Date().toISOString(),
            });
          }
          case "/edge": {
            return jsonResponse({
              colo: request.cf?.colo,
              country: request.cf?.country,
              city: request.cf?.city,
              asn: request.cf?.asn,
              httpProtocol: request.cf?.httpProtocol,
              tlsVersion: request.cf?.tlsVersion,
              clientIp: request.headers.get("CF-Connecting-IP") ?? "unknown",
              timestamp: new Date().toISOString(),
            });
          }
          case "/config": {
            return jsonResponse({
              app: env.APP_NAME,
              course: env.COURSE_NAME,
              secrets: {
                apiTokenSet: Boolean(env.API_TOKEN),
                adminEmailSet: Boolean(env.ADMIN_EMAIL),
              },
            });
          }
          case "/counter": {
            const raw = await env.SETTINGS.get("visits");
            const visits = Number(raw ?? "0") + 1;
            await env.SETTINGS.put("visits", String(visits));
            return jsonResponse({ visits, source: "workers-kv" });
          }
          default: {
            break;
          }
        }
        break;
      }
      case "POST": {
        switch (url.pathname) {
          case "/counter/reset": {
            await env.SETTINGS.put("visits", "0");
            return jsonResponse({ reset: true, visits: 0 });
          }
          default: {
            break;
          }
        }
        break;
      }
      default: {
        break;
      }
    }
    return jsonResponse(
      { error: "Not Found", path: url.pathname },
      { status: 404 },
    );
  },
};
