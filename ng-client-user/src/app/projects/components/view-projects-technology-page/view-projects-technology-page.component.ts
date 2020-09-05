import { Component, OnInit } from '@angular/core';
import { Project } from 'src/app/models/api/project';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-view-projects-technology-page',
  templateUrl: './view-projects-technology-page.component.html',
  styleUrls: ['./view-projects-technology-page.component.scss'],
})
export class ViewProjectsTechnologyPageComponent implements OnInit {
  technologyName: string;
  projectsList: Project[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private projectService: ProjectService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      this.technologyName = params.technologyName;
      if (isNaN(+params.page) === false) {
        this.page = +params.page;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.pageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.projectService.getAllProjects(this.page, this.pageSize).subscribe((result) => {
      this.projectsList = result.data;
    });
  }
}
