import { Component, OnInit, OnDestroy, Inject } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { Project } from '../../../models/api/project';
import { ActivatedRoute } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';

@Component({
  selector: 'app-view-projects-page',
  templateUrl: './view-projects-page.component.html',
  styleUrls: ['./view-projects-page.component.scss'],
})
export class ViewProjectsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
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
    this.title.setTitle(`Loading Projects | Axmouth's Website`);
    this.loading = true;
    this.projectService
      .getAllProjects(this.page, this.pageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.projectsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Projects Index | Axmouth's Website`);
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
