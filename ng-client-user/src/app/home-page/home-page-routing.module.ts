import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeMainComponent } from './components/home-main/home-main.component';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { HomePageComponent } from './components/home-page/home-page.component';
import { MyPageComponent } from './components/my-page/my-page.component';

const routes: Routes = [
  {
    path: '',
    component: HomeMainComponent,

    children: [
      { path: '', component: HomePageComponent, pathMatch: 'full' },
      { path: 'home', component: MyPageComponent, redirectTo: '/' },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class HomePageRoutingModule {}
