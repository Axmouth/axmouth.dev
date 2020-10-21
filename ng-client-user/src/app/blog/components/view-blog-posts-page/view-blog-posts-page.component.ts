import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlogPostService } from '../../services/blog-post.service';
import { BlogPost } from 'src/app/models/api/blog-post';
import { ActivatedRoute, Router } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';
import { Inject } from '@angular/core';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-view-blog-posts-page',
  templateUrl: './view-blog-posts-page.component.html',
  styleUrls: ['./view-blog-posts-page.component.scss'],
})
export class ViewBlogPostsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
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
      console.log(this.page);
      console.log(this.page);
      this.sortType = qParams.sortType ?? 'CreatedAtDesc';
      this.initialiseState();
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Blog Posts | Axmouth's Website`);
    this.loading = true;
    this.blogPostService
      .getAllPosts({ page: this.page, pageSize: this.pageSize, sortType: this.sortType })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.blogPostsList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Blog Posts Index | Axmouth's Website`);
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
