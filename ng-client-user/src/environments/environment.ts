// This file can be replaced during build by using the `fileReplacements` array.
// `ng build --prod` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,
};

export const apiRoot = 'http://127.0.0.1:39051/api/v1';
export const apiRootServer = 'http://127.0.0.1:39051/api/v1';
export const websiteUrl = 'https://axmouth.dev';
export const jwtWhitelist = ['localhost', 'localhost:39051', 'localhost:4200', '127.0.0.1:39051', '127.0.0.1'];
export const HCAPTCHA_SITE_KEY = '10000000-ffff-ffff-ffff-000000000001';
/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/dist/zone-error';  // Included with Angular CLI.
