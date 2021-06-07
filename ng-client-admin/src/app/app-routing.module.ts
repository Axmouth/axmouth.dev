import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { AuthGuard } from '../auth/guards/auth.guard';
import { GuestGuard } from '../auth/guards/guest.guard';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { HomeComponent } from './components/home/home.component';
import { NotFoundPageComponent } from './admin-dashboard/components/not-found-page/not-found-page.component';
import { ViewAdminCategoryComponent } from './admin-dashboard/components/view-admin-category/view-admin-category.component';
import { ViewAdminModelComponent } from './admin-dashboard/components/view-admin-model/view-admin-model.component';
import { ViewEntityComponent } from './admin-dashboard/components/view-entity/view-entity.component';
import { CreateEntityComponent } from './admin-dashboard/components/create-entity/create-entity.component';
import { AdminLogDetailsComponent } from './components/pages/admin-log-details/admin-log-details.component';
import { AdminLogsComponent } from './components/pages/admin-logs/admin-logs.component';

const routes: Routes = [
  { path: '', component: HomeComponent, pathMatch: 'full', canActivate: [AuthGuard] },
  { path: 'admin-logs', component: AdminLogsComponent, canActivate: [AuthGuard] },
  { path: 'admin-logs/:adminLogId', component: AdminLogDetailsComponent, canActivate: [AuthGuard] },
  {
    path: '',
    loadChildren: () => import('./admin-dashboard/admin-dashboard.module').then((m) => m.AdminDashboardModule),
  },
  { path: 'login', component: LoginPageComponent, canActivate: [GuestGuard] },
  { path: 'logout', component: LogoutPageComponent, canActivate: [AuthGuard] },
  { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, {
      initialNavigation: 'enabled',
      relativeLinkResolution: 'legacy',
    }),
  ],
  exports: [RouterModule],
})
export class RoutingModule {}
