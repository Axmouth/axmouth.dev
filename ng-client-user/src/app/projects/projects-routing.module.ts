import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { ViewProjectsPageComponent } from './components/view-projects-page/view-projects-page.component';
import { ViewProjectDetailsPageComponent } from './components/view-project-details-page/view-project-details-page.component';
import { ProjectsMainComponent } from './components/projects-main/projects-main.component';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { ViewProjectsTechnologyPageComponent } from './components/view-projects-technology-page/view-projects-technology-page.component';

const routes: Routes = [
  {
    path: '',
    component: ProjectsMainComponent,

    children: [
      { path: '', component: ViewProjectsPageComponent, pathMatch: 'full' },
      { path: 'technology/:technologyName', component: ViewProjectsTechnologyPageComponent, pathMatch: 'full' },
      { path: ':id', component: ViewProjectDetailsPageComponent, pathMatch: 'full' },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProjectsRoutingModule {}
