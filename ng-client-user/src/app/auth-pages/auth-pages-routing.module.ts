import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { LoginPageComponent } from './pages/login-page/login-page.component';
import { GuestGuard } from 'src/auth/guards/guest.guard';
import { LogoutPageComponent } from './pages/logout-page/logout-page.component';
import { AuthGuard } from 'src/auth/guards/auth.guard';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { AuthMainComponent } from './components/auth-main/auth-main.component';
import { RegisterPageComponent } from './pages/register-page/register-page.component';
import { PasswordChangePageComponent } from './pages/password-change-page/password-change-page.component';
import { VerifyEmailPageComponent } from './pages/verify-email-page/verify-email-page.component';
import { SettingsPageComponent } from './pages/settings-page/settings-page.component';
import { RequestPasswordChangePageComponent } from './pages/request-password-change-page/request-password-change-page.component';

const routes: Routes = [
  {
    path: '',
    component: AuthMainComponent,

    children: [
      { path: 'login', component: LoginPageComponent, canActivate: [GuestGuard] },
      { path: 'register', component: RegisterPageComponent, canActivate: [GuestGuard] },
      { path: 'logout', component: LogoutPageComponent, canActivate: [AuthGuard] },
      { path: 'settings', component: SettingsPageComponent, canActivate: [AuthGuard] },
      { path: 'verify-email', component: VerifyEmailPageComponent },
      { path: 'reset-password', component: PasswordChangePageComponent },
      { path: 'request-password-reset', component: RequestPasswordChangePageComponent },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class AuthPagesRoutingModule {}
