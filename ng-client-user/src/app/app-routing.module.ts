import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { NotFoundPageComponent } from 'src/app/shared/components/not-found-page/not-found-page.component';

const routes: Routes = [
  { path: 'projects', loadChildren: () => import('./projects/projects.module').then((m) => m.ProjectsModule) },
  { path: 'blog', loadChildren: () => import('./blog/blog.module').then((m) => m.BlogModule) },
  { path: 'contact', loadChildren: () => import('./contact/contact.module').then((m) => m.ContactModule) },
  { path: 'auth', loadChildren: () => import('./auth-pages/auth-pages.module').then((m) => m.AuthPagesModule) },
  { path: 'login', redirectTo: '/auth/login' },
  { path: 'search', loadChildren: () => import('./search/search.module').then((m) => m.SearchModule) },
  { path: '', loadChildren: () => import('./home-page/home-page.module').then((m) => m.HomePageModule) },
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
