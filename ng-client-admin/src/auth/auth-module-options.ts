import { Provider } from '@angular/core';
import { AuthModuleOptionsConfig } from './auth-module-options-config';

export interface AuthModuleOptions {
  jwtOptionsProvider?: Provider;
  config?: AuthModuleOptionsConfig;
}
