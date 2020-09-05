import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { ProjectTechnology } from '../../../models/api/project-technology';
import { TechnologyService } from '../../services/technology.service';

@Component({
  selector: 'app-projects-technologies-side-widget',
  templateUrl: './projects-technologies-side-widget.component.html',
  styleUrls: ['./projects-technologies-side-widget.component.scss'],
})
export class ProjectsTechnologiesSideWidgetComponent implements OnInit {
  projectTechnologiesList: ProjectTechnology[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private technologyService: TechnologyService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      if (isNaN(+params.page) === false) {
        this.page = +params.categoryPage;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.categoryPageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.technologyService.getAllTechnologies(this.page, this.pageSize).subscribe((result) => {
      this.projectTechnologiesList = result.data;
      this.resultNumber = result?.pagination?.totalResults;
    });
  }
}
