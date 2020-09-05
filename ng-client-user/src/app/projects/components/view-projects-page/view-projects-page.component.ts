import { Component, OnInit } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { Project } from '../../../models/api/project';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-view-projects-page',
  templateUrl: './view-projects-page.component.html',
  styleUrls: ['./view-projects-page.component.scss'],
})
export class ViewProjectsPageComponent implements OnInit {
  projectsList: Project[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private projectService: ProjectService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
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
