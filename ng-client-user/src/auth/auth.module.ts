import { NgModule, Provider, Optional, SkipSelf, ModuleWithProviders } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HTTP_INTERCEPTORS } from '@angular/common/http';
import { JwtInterceptor } from '.';
import { AX_AUTH_OPTIONS } from './auth-injection-token';
import { AuthService } from './services/auth.service';
import { AuthModuleOptions } from './auth-module-options';

@NgModule({
  declarations: [],
  imports: [CommonModule],
})
export class AuthModule {
  constructor(@Optional() @SkipSelf() parentModule: AuthModule) {
    if (parentModule) {
      throw new Error('AuthModule is already loaded. You only need to load once, FOOL!');
    }
  }

  static forRoot(options: AuthModuleOptions): ModuleWithProviders<AuthModule> {
    return {
      ngModule: AuthModule,
      providers: [
        {
          provide: HTTP_INTERCEPTORS,
          useClass: JwtInterceptor,
          multi: true,
        },
        options.jwtOptionsProvider || {
          provide: AX_AUTH_OPTIONS,
          useValue: options.config,
        },
        AuthService,
      ],
    };
  }
}
