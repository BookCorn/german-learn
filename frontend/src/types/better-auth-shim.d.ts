declare module "better-auth" {
  // Minimal shim to satisfy type-check during UI development
  export function betterAuth(config?: any): any;
}

declare module "better-auth/sveltekit" {
  export const svelteKit: any;
  export const handleHooks: any;
}
