import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AuthPagesRoutingModule } from './auth-pages-routing.module';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { AuthMainComponent } from './components/auth-main/auth-main.component';
import { RegisterPageComponent } from './components/register-page/register-page.component';
import { SharedModule } from '../shared/shared.module';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { VerifyEmailPageComponent } from './components/verify-email-page/verify-email-page.component';
import { RequestPasswordChangePageComponent } from './components/request-password-change-page/request-password-change-page.component';
import { PasswordChangePageComponent } from './components/password-change-page/password-change-page.component';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { SettingsPageComponent } from './components/settings-page/settings-page.component';

@NgModule({
  declarations: [
    LoginPageComponent,
    LogoutPageComponent,
    AuthMainComponent,
    RegisterPageComponent,
    VerifyEmailPageComponent,
    RequestPasswordChangePageComponent,
    PasswordChangePageComponent,
    SettingsPageComponent,
  ],
  imports: [CommonModule, AuthPagesRoutingModule, SharedModule, ReactiveFormsModule, FormsModule, NgbModule],
  providers: [],
})
export class AuthPagesModule {}
