import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { RoutingModule } from './app-routing.module';
import { FlexLayoutModule } from '@angular/flex-layout/';
import { TransferHttpCacheModule } from '@nguniversal/common';

import { AppComponent } from './app.component';
import { AuthModule } from 'src/auth/auth.module';
import { apiRoot } from 'src/environments/environment';
import { HttpClientModule, HTTP_INTERCEPTORS, HttpClient } from '@angular/common/http';
import { LinksSideWidgetComponent } from './shared/components/links-side-widget/links-side-widget.component';
import { SharedModule } from './shared/shared.module';
import { HCaptchaDirective } from './shared/directives/h-captcha.directive';
import { NgbModule, NgbAlertModule } from '@ng-bootstrap/ng-bootstrap';
import { jwtWhitelist } from 'src/environments/environment';
import { JwtInterceptor } from 'src/auth';
import { FooterComponent } from './components/common/footer/footer.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@NgModule({
  declarations: [AppComponent, FooterComponent],
  imports: [
    BrowserModule.withServerTransition({ appId: 'serverApp' }),
    HttpClientModule,
    RoutingModule,
    SharedModule,
    FlexLayoutModule,
    TransferHttpCacheModule,
    AuthModule.forRoot({
      config: {
        authEndpointPrefix: `${apiRoot}/auth/`,
        whitelistedDomains: jwtWhitelist,
        blacklistedRoutes: [],
        skipWhenExpired: false,
        passwordResetEndpoint: 'reset-password',
        requestPasswordResetEndpoint: 'request-password-reset',
        verifyEmailEndpoint: 'verify-email',
        requestVerificationEmailEndpoint: 'request-verification-email',
        userNameJwtKey: 'display_name',
        passwordResetConfig: {
          tokenKey: 'token',
          emailKey: 'email',
          emailQueryKey: 'email',
          tokenQueryKey: 'token',
          userNameKey: 'username',
          userNameQueryKey: 'username',
        },
        verifyEmailConfig: {
          tokenKey: 'token',
          emailKey: 'email',
          emailQueryKey: 'email',
          tokenQueryKey: 'token',
          userNameKey: 'username',
          userNameQueryKey: 'username',
        },
      },
    }),
    NgbModule,
    FontAwesomeModule,
  ],
  providers: [
    {
      provide: HTTP_INTERCEPTORS,
      useClass: JwtInterceptor,
      multi: true,
    },
  ],
  bootstrap: [AppComponent],
})
export class AppModule {}
