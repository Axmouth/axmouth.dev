import { Component, OnInit, Input } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute } from '@angular/router';
import { Project } from '../../../models/api/project';
import { Title } from '@angular/platform-browser';

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
  notFound = false;
  loading = true;

  constructor(private projectService: ProjectService, private route: ActivatedRoute, private title: Title) {}

  ngOnInit(): void {
    this.title.setTitle('axmouth.dev - Loading Project');
    const params = this.route.snapshot.paramMap;
    this.projectId = params.get('id');
    this.projectService.getProject(this.projectId).subscribe(
      (result) => {
        this.project = result.data;
        this.projectBodyData = JSON.parse(result.data.body);
        this.projectTechnologiesList = result.data.technologies;
        this.loading = false;
        this.title.setTitle(`axmouth.dev - ${this.project.name}`);
      },
      (error) => {
        console.log(error);
        if (error.status === 404) {
          this.notFound = true;
          this.title.setTitle('axmouth.dev - Project Not Found');
        }
        this.loading = false;
      },
    );
  }
}
