import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { AuthGuard } from '../auth/guards/auth.guard';
import { GuestGuard } from '../auth/guards/guest.guard';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { HomeComponent } from './components/home/home.component';
import { NotFoundPageComponent } from './components/not-found-page/not-found-page.component';
import { ViewAdminCategoryComponent } from './components/view-admin-category/view-admin-category.component';
import { ViewAdminModelComponent } from './components/view-admin-model/view-admin-model.component';
import { ViewEntityComponent } from './components/view-entity/view-entity.component';
import { CreateEntityComponent } from './components/create-entity/create-entity.component';

const routes: Routes = [
  { path: '', component: HomeComponent, pathMatch: 'full', canActivate: [AuthGuard] },
  { path: 'categories/:categoryName', component: ViewAdminCategoryComponent, canActivate: [AuthGuard] },
  { path: 'categories/:categoryName/models/:modelName', component: ViewAdminModelComponent, canActivate: [AuthGuard] },
  {
    path: 'categories/:categoryName/models/:modelName/add',
    component: CreateEntityComponent,
    canActivate: [AuthGuard],
  },
  { path: 'categories/:categoryName/models/:modelName/:id', component: ViewEntityComponent, canActivate: [AuthGuard] },
  { path: 'login', component: LoginPageComponent, canActivate: [GuestGuard] },
  { path: 'logout', component: LogoutPageComponent, canActivate: [AuthGuard] },
  { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, {
      initialNavigation: 'enabled',
    }),
  ],
  exports: [RouterModule],
})
export class RoutingModule {}
