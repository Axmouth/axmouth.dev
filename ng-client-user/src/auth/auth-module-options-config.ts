/**
 * Interface for classes that represent options to customize Authentication Module.
 *
 */
export interface AuthModuleOptionsConfig {
  /**
   * A prefix applied to all endpoints used in this module.
   *
   */
  authEndpointPrefix: string;
  /**
   * The header that the JWT Interceptor uses to add the token to requests.
   *
   */
  headerName?: string;
  /**
   * A prefix added to the token, defaults to Bearer.
   *
   */
  authScheme?: string;
  /**
   * A whitelist for the JWT Interceptor to add the token for.
   *
   */
  whitelistedDomains?: Array<string | RegExp>;
  /**
   * A blacklist for the JWT Interceptor to avoid adding the token for.
   *
   */
  blacklistedRoutes?: Array<string | RegExp>;
  /**
   * Whether to throw an error for invalid tokens.
   *
   */
  throwNoTokenError?: boolean;
  /**
   * The localstorage key to save the token to, defaults to 'auth_app_token'.
   *
   */
  jwtTokenKey?: string;
  /**
   * Whether to add the token to requests once it is expired.
   *
   */
  skipWhenExpired?: boolean;
  /**
   * Endpoint to use for logging in, preceded by authEndpointPrefix. Defaults to 'login'.
   *
   */
  loginEndpoint?: string;
  /**
   * HTTP method to use for logging in.
   * Defaults to 'post'.
   *
   */
  loginMethod?: HttpMethod;
  /**
   * Function to use to get the new JWT token out of a login response.
   *
   */
  loginTokengetter?: <T>(authRes: T) => string;
  /**
   * Endpoint to use for logging out, preceded by authEndpointPrefix.
   * Defaults to getting data.token from the response.
   *
   */
  logoutEndpoint?: string;
  /**
   * HTTP method to use for logging out.
   * Defaults to 'delet'.
   *
   */
  logoutMethod?: HttpMethod;
  /**
   * Endpoint to use for refreshing the JWT, preceded by authEndpointPrefix.
   * Defaults to 'refresh'.
   *
   */
  refreshEndpoint?: string;
  /**
   * HTTP method to use for refreshing the JWT token.
   * Defaults to 'post'.
   *
   */
  refreshMethod?: HttpMethod;
  /**
   * Function to use to get the new JWT token out of a refresh response.
   *
   */
  refreshTokengetter?: <T>(authRes: T) => string;
  /**
   * Endpoint to use for registering, preceded by authEndpointPrefix.
   * Defaults to getting data.token from the response.
   *
   */
  registerEndpoint?: string;
  /**
   * HTTP method to use for registering.
   * Defaults to 'post'.
   *
   */
  registerMethod?: HttpMethod;
  /**
   * Function to use to get the new JWT token out of a register response.
   *
   */
  registerTokengetter?: <T>(authRes: T) => string;
  /**
   * Endpoint to use for getting the user profile, preceded by authEndpointPrefix.
   * Defaults to getting data.token from the response.
   *
   */
  profileEndpoint?: string;
  /**
   * HTTP method to use for getting a users profile.
   * Defaults to 'get'.
   *
   */
  profileMethod?: HttpMethod;
  /**
   * Endpoint to use for requesting a password reset, preceded by authEndpointPrefix.
   * Defaults to 'request-password-reset'.
   *
   */
  requestPasswordResetEndpoint?: string;
  /**
   * HTTP method to use for requesting a password reset.
   * Defaults to 'post'.
   *
   */
  requestPasswordResetMethod?: HttpMethod;
  /**
   * Endpoint to use for resetting password, preceded by authEndpointPrefix.
   * Defaults to 'password-reset'.
   *
   */
  passwordResetEndpoint?: string;
  /**
   * HTTP method to use for resetting a password.
   * Defaults to 'post'.
   *
   */
  passwordResetMethod?: HttpMethod;
  /**
   * Endpoint to use for verifying email addresses(links to this should only be
   * supplied through verification emails), preceded by authEndpointPrefix.
   * Defaults to 'verify-email'.
   *
   */
  verifyEmailEndpoint?: string;
  /**
   * HTTP method to use for verifying an email.
   * Defaults to 'post'.
   *
   */
  verifyEmailMethod?: HttpMethod;
  /**
   * Endpoint to use for requesting a verification email, preceded by authEndpointPrefix.
   * Defaults to 'request-verification-email'.
   *
   */
  requestVerificationEmailEndpoint?: string;
  /**
   * HTTP method to use for requesting a verification email.
   * Defaults to 'post'.
   *
   */
  requestVerificationEmailMethod?: HttpMethod;
  /**
   * Configuration for the key where user email is stored within the JWT.
   *
   */
  emailJwtKey?: string;
  /**
   * Configuration for the key where user name is stored within the JWT.
   *
   */
  userNameJwtKey?: string;
  /**
   * Configuration for the query keys used by the password reset function.
   *
   */
  passwordResetConfig?: PasswordResetConfig;
  /**
   * Configuration for the query keys used by the verify email function.
   *
   */
  verifyEmailConfig?: VerifyEmailConfig;
}

/**
 * Configuration for the query keys used by the password reset function.
 *
 */
export interface PasswordResetConfig {
  /**
   * Query key for password reset token.
   * Defaults to 'reset_password_token'.
   *
   */
  tokenQueryKey: string;
  /**
   * Query key for username.
   * Defaults to 'userName'.
   *
   */
  userNameQueryKey: string;
  /**
   * Query key for email.
   * Defaults to 'email'.
   *
   */
  emailQueryKey: string;
  /**
   * JSON response key for password reset token.
   * Defaults to 'token'.
   *
   */
  tokenKey: string;
  /**
   * JSON response key for username.
   * Defaults to 'userName'.
   *
   */
  userNameKey: string;
  /**
   * JSON response key for email.
   * Defaults to 'email'.
   *
   */
  emailKey: string;
}

/**
 * Configuration for the query keys used by the verify email function.
 *
 */
export interface VerifyEmailConfig {
  /**
   * Query key for email verification token.
   * Defaults to 'email_confirm_token'.
   *
   */
  tokenQueryKey: string;
  /**
   * Query key for username.
   * Defaults to 'userName'.
   *
   */
  userNameQueryKey: string;
  /**
   * Query key for email.
   * Defaults to 'email'.
   *
   */
  emailQueryKey: string;
  /**
   * JSON response key for email verification token.
   * Defaults to 'token'.
   *
   */
  tokenKey: string;
  /**
   * JSON response key for username.
   * Defaults to 'userName'.
   *
   */
  userNameKey: string;
  /**
   * JSON response key for email.
   * Defaults to 'email'.
   *
   */
  emailKey: string;
}

/**
 * Type used to pass HTTP method type as a parameter
 *
 */
export type HttpMethod = 'get' | 'post' | 'put' | 'patch' | 'delete';
