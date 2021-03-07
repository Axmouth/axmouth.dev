import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit, OnDestroy } from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { ActivatedRoute, Router } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { ProjectService } from 'src/app/projects/services/project.service';
import { websiteUrl } from 'src/environments/environment';
import { BlogPost } from '../../../models/api/blog-post';
import { BlogPostService } from '../../services/blog-post.service';

@Component({
  selector: 'app-view-blog-post-search-page',
  templateUrl: './view-blog-post-search-page.component.html',
  styleUrls: ['./view-blog-post-search-page.component.scss'],
})
export class ViewBlogPostSearchPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  resultNumber = 0;
  searchQuery: string;
  blogPostsList: BlogPost[] = [];
  page: number;
  pageSize: number;
  sortType: string;
  loading = true;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private blogPostsService: BlogPostService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.searchQuery = params.search;
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
    this.blogPostsService
      .getAllPosts({
        page: this.page,
        pageSize: this.pageSize,
        sortType: this.sortType,
        search: this.searchQuery,
      })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.blogPostsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Blog, Search: ${this.searchQuery} | Axmouth's Website`);
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
