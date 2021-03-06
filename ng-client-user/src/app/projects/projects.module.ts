import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ProjectsRoutingModule } from './projects-routing.module';
import { ViewProjectsPageComponent } from './pages/view-projects-page/view-projects-page.component';
import { ViewProjectDetailsPageComponent } from './pages/view-project-details-page/view-project-details-page.component';
import { ProjectsMainComponent } from './components/projects-main/projects-main.component';
import { SharedModule } from '../shared/shared.module';
import { ProjectsTechnologiesSideWidgetComponent } from './components/projects-technologies-side-widget/projects-technologies-side-widget.component';
import { ProjectPreviewComponent } from './components/project-preview/project-preview.component';
import { ViewProjectsTechnologyPageComponent } from './pages/view-projects-technology-page/view-projects-technology-page.component';
import { ProjectService } from './services/project.service';
import { TechnologyService } from './services/technology.service';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { FormsModule } from '@angular/forms';
import { ProjectPreviewPlaceholderComponent } from './components/project-preview-placeholder/project-preview-placeholder.component';
import { ProjectDetailsPlaceholderComponent } from './components/project-details-placeholder/project-details-placeholder.component';

@NgModule({
  declarations: [
    ViewProjectsPageComponent,
    ViewProjectDetailsPageComponent,
    ProjectsMainComponent,
    ProjectsTechnologiesSideWidgetComponent,
    ProjectPreviewComponent,
    ViewProjectsTechnologyPageComponent,
    ProjectPreviewPlaceholderComponent,
    ProjectDetailsPlaceholderComponent,
  ],
  imports: [CommonModule, ProjectsRoutingModule, SharedModule, NgbModule, FormsModule],
  providers: [ProjectService, TechnologyService],
})
export class ProjectsModule {}
