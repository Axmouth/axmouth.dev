import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlogPost } from '../../../models/api/blog-post';
import { BlogPostService } from '../../services/blog-post.service';
import { ActivatedRoute, Router } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';
import { DOCUMENT, Location } from '@angular/common';
import { Inject } from '@angular/core';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-view-blog-posts-category-page',
  templateUrl: './view-blog-posts-category-page.component.html',
  styleUrls: ['./view-blog-posts-category-page.component.scss'],
})
export class ViewBlogPostsCategoryPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  categoryName: string;
  resultNumber = 0;
  blogPostsList: BlogPost[] = [];
  page: number;
  pageSize: number;
  sortType: string;
  loading = true;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private blogPostService: BlogPostService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.categoryName = params.categoryName;
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
    this.title.setTitle(`Loading Blog Posts, Category: ${this.categoryName} | Axmouth's Website`);
    this.loading = true;
    this.blogPostService
      .getAllPosts({
        categoryName: this.categoryName,
        page: this.page,
        pageSize: this.pageSize,
        sortType: this.sortType,
      })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.blogPostsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Blog Posts Index, Category: ${this.categoryName} | Axmouth's Website`);
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
