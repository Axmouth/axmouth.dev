import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { GuestGuard } from 'src/auth/guards/guest.guard';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { AuthGuard } from 'src/auth/guards/auth.guard';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { AuthMainComponent } from './components/auth-main/auth-main.component';
import { RegisterPageComponent } from './components/register-page/register-page.component';

const routes: Routes = [
  {
    path: '',
    component: AuthMainComponent,

    children: [
      { path: 'login', component: LoginPageComponent, canActivate: [GuestGuard] },
      { path: 'register', component: RegisterPageComponent, canActivate: [GuestGuard] },
      { path: 'logout', component: LogoutPageComponent, canActivate: [AuthGuard] },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class AuthPagesRoutingModule {}
