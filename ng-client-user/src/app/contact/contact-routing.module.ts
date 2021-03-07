import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { ContactMainComponent } from './components/contact-main/contact-main.component';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { ContactSuccessPageComponent } from './pages/contact-success-page/contact-success-page.component';
import { ContactPageComponent } from './pages/contact-page/contact-page.component';

const routes: Routes = [
  {
    path: '',
    component: ContactMainComponent,

    children: [
      { path: '', component: ContactPageComponent, pathMatch: 'full' },
      { path: 'success', component: ContactSuccessPageComponent, pathMatch: 'full' },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ContactRoutingModule {}
