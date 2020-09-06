import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { RoutingModule } from './app-routing.module';
import { FlexLayoutModule } from '@angular/flex-layout';

import { AppComponent } from './app.component';
import { AuthModule } from 'src/auth/auth.module';
import { apiRoot } from 'src/environments/environment';
import { HttpClientModule } from '@angular/common/http';
import { LinksSideWidgetComponent } from './shared/components/links-side-widget/links-side-widget.component';
import { SharedModule } from './shared/shared.module';
import { HCaptchaDirective } from './shared/directives/h-captcha.directive';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';

@NgModule({
  declarations: [AppComponent],
  imports: [
    BrowserModule,
    HttpClientModule,
    RoutingModule,
    SharedModule,
    FlexLayoutModule,
    AuthModule.forRoot({
      config: {
        authEndpointPrefix: `${apiRoot}/auth/`,
        whitelistedDomains: [
          'localhost',
          'localhost:39051',
          '[::1]:39051',
          'localhost:4200',
          'axmouth.dev',
          'admin.axmouth.dev',
          'backend.axmouth.dev',
        ],
        blacklistedRoutes: [],
        // authScheme: ""
      },
    }),
    NgbModule,
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
