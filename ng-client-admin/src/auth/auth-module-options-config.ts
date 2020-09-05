export interface AuthModuleOptionsConfig {
  authEndpointPrefix: string;
  headerName?: string;
  authScheme?: string;
  whitelistedDomains?: Array<string | RegExp>;
  blacklistedRoutes?: Array<string | RegExp>;
  throwNoTokenError?: boolean;
  skipWhenExpired?: boolean;
}
