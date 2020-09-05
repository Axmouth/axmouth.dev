import { Component, OnInit, Input } from '@angular/core';
import { Project } from 'src/app/models/api/project';

@Component({
  selector: 'app-project-preview',
  templateUrl: './project-preview.component.html',
  styleUrls: ['./project-preview.component.scss'],
})
export class ProjectPreviewComponent implements OnInit {
  @Input()
  project: Project;
  projectTechnologiesList: string[] = [];

  constructor() {}

  ngOnInit(): void {
    this.projectTechnologiesList = this.project.technologies;
  }
}
