import { Component, OnInit, OnDestroy, Inject } from '@angular/core';
import { Project } from 'src/app/models/api/project';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';
import { DOCUMENT } from '@angular/common';

@Component({
  selector: 'app-view-projects-technology-page',
  templateUrl: './view-projects-technology-page.component.html',
  styleUrls: ['./view-projects-technology-page.component.scss'],
})
export class ViewProjectsTechnologyPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  technologyName: string;
  projectsList: Project[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;
  loading = true;

  constructor(
    private projectService: ProjectService,
    private route: ActivatedRoute,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
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
    this.title.setTitle(`Loading Projects - Axmouth's Website`);
    this.loading = true;
    this.projectService
      .getAllProjectsByTechnology(this.technologyName, this.page, this.pageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.projectsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Projects, Category: ${this.technologyName} - Axmouth's Website`);
        this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
        this.meta.updateTag({ property: `og:url`, content: this.doc.location.href });
        this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
        this.meta.updateTag({ property: `twitter:url`, content: this.doc.location.href });
        this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
