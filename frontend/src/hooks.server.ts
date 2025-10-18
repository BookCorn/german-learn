import type { Handle } from "@sveltejs/kit";

// No-op server handle (BetterAuth disabled for now to avoid missing subpath issues)
export const handle: Handle = async ({ event, resolve }) => {
  return resolve(event);
};
