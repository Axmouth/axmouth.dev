import { Component, OnInit, OnDestroy, Inject } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { Project } from '../../../models/api/project';
import { ActivatedRoute, Router } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-view-projects-page',
  templateUrl: './view-projects-page.component.html',
  styleUrls: ['./view-projects-page.component.scss'],
})
export class ViewProjectsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  projectsList: Project[];
  resultNumber = 0;
  page: number;
  pageSize: number;
  sortType: string;
  loading = true;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private projectService: ProjectService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.route.queryParams.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      if (isNaN(+params.page) === false) {
        this.page = +params.page ?? 1;
      } else {
        this.page = 1;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.pageSize ?? 5;
      } else {
        this.pageSize = 5;
      }
      this.sortType = params.sortType ?? 'CreatedAtDesc';
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Projects | Axmouth's Website`);
    this.loading = true;
    this.projectService
      .getAllProjects({ page: this.page, pageSize: this.pageSize, sortType: this.sortType })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.projectsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Projects Index | Axmouth's Website`);
        this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
        this.meta.updateTag({
          property: `og:url`,
          content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
        });
        this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
        this.meta.updateTag({
          property: `twitter:url`,
          content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
        });
        this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
      });
  }

  onPageChange(): void {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: { page: this.page, pageSize: this.pageSize, sortType: this.sortType },
      queryParamsHandling: 'merge',
    });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
