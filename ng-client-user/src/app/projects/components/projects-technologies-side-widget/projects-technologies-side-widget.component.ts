import { Component, OnInit, OnDestroy } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { ProjectTechnology } from '../../../models/api/project-technology';
import { TechnologyService } from '../../services/technology.service';

@Component({
  selector: 'app-projects-technologies-side-widget',
  templateUrl: './projects-technologies-side-widget.component.html',
  styleUrls: ['./projects-technologies-side-widget.component.scss'],
})
export class ProjectsTechnologiesSideWidgetComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  projectTechnologiesList: ProjectTechnology[];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private technologyService: TechnologyService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
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
    this.technologyService
      .getAllTechnologies(this.page, this.pageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.projectTechnologiesList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
