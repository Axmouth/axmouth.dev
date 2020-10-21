import { Component, OnInit, OnDestroy, Inject } from '@angular/core';
import { Project } from 'src/app/models/api/project';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute, Router } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';
import { DOCUMENT } from '@angular/common';
import { websiteUrl } from 'src/environments/environment';

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
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.technologyName = params.technologyName;
      this.route.queryParams.pipe(takeUntil(this.ngUnsubscribe)).subscribe((qParams) => {
        if (isNaN(+qParams.page) === false) {
          this.page = +qParams.page ?? 1;
        } else {
          this.page = 1;
        }
        if (isNaN(+qParams.pageSize) === false) {
          this.pageSize = +qParams.pageSize ?? 5;
        } else {
          this.pageSize = 5;
        }
        this.sortType = qParams.sortType ?? 'CreatedAtDesc';
        this.initialiseState();
      });
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Projects | Axmouth's Website`);
    this.loading = true;
    this.projectService
      .getAllProjects({
        page: this.page,
        pageSize: this.pageSize,
        sortType: this.sortType,
        technologyName: this.technologyName,
      })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.projectsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Projects, Category: ${this.technologyName} | Axmouth's Website`);
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
