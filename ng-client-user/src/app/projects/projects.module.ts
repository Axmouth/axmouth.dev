import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ProjectsRoutingModule } from './projects-routing.module';
import { ViewProjectsPageComponent } from './components/view-projects-page/view-projects-page.component';
import { ViewProjectDetailsPageComponent } from './components/view-project-details-page/view-project-details-page.component';
import { ProjectsMainComponent } from './components/projects-main/projects-main.component';
import { SharedModule } from '../shared/shared.module';
import { ProjectsTechnologiesSideWidgetComponent } from './components/projects-technologies-side-widget/projects-technologies-side-widget.component';
import { ProjectPreviewComponent } from './components/project-preview/project-preview.component';
import { ViewProjectsTechnologyPageComponent } from './components/view-projects-technology-page/view-projects-technology-page.component';
import { ProjectService } from './services/project.service';
import { TechnologyService } from './services/technology.service';

@NgModule({
  declarations: [
    ViewProjectsPageComponent,
    ViewProjectDetailsPageComponent,
    ProjectsMainComponent,
    ProjectsTechnologiesSideWidgetComponent,
    ProjectPreviewComponent,
    ViewProjectsTechnologyPageComponent,
  ],
  imports: [CommonModule, ProjectsRoutingModule, SharedModule],
  providers: [ProjectService, TechnologyService],
})
export class ProjectsModule {}
