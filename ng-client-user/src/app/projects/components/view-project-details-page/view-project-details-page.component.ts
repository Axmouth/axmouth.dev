import { Component, OnInit, Input } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute } from '@angular/router';
import { Project } from '../../../models/api/project';

@Component({
  selector: 'app-view-project-details-page',
  templateUrl: './view-project-details-page.component.html',
  styleUrls: ['./view-project-details-page.component.scss'],
})
export class ViewProjectDetailsPageComponent implements OnInit {
  @Input()
  project: Project;
  projectTechnologiesList: string[] = [];
  projectId: string;
  projectBodyData: any[];

  constructor(private projectService: ProjectService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.projectId = params.get('id');
    this.projectService.getProject(this.projectId).subscribe((result) => {
      this.project = result.data;
      this.projectBodyData = JSON.parse(result.data.body);
      this.projectTechnologiesList = result.data.technologies;
    });
  }
}
