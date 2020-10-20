import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlogPostService } from '../../services/blog-post.service';
import { ActivatedRoute } from '@angular/router';
import { BlogPost } from '../../../models/api/blog-post';
import { BlogPostCommentService } from '../../services/blog-post-comment.service';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';
import { Inject } from '@angular/core';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-view-blog-post-details-page',
  templateUrl: './view-blog-post-details-page.component.html',
  styleUrls: ['./view-blog-post-details-page.component.scss'],
})
export class ViewBlogPostDetailsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  post: BlogPost;
  commentsCount = 0;
  postId: string;
  postBodyData: any[];
  blogPostComments: BlogPostComment[] = [];
  commentPage: number;
  commentPageSize: number;
  notFound = false;
  loading = true;

  constructor(
    private blogPostService: BlogPostService,
    private commentService: BlogPostCommentService,
    private route: ActivatedRoute,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Loading Blog Post | Axmouth's Website`);
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.postId = params.id;
      if (isNaN(+params.page) === false) {
        this.commentPage = +params.commentPage;
      }
      if (isNaN(+params.pageSize) === false) {
        this.commentPageSize = +params.commentPageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.blogPostService.getPost(this.postId).subscribe((result) => {
      this.post = result.data;
      this.postBodyData = JSON.parse(result.data.body);
      this.loading = false;
      this.title.setTitle(`${this.post.title} | Axmouth's Website`);
      this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
      this.meta.updateTag({ name: `description`, content: this?.post?.description });
      this.meta.updateTag({
        name: `keywords`,
        content: `axmouth,developer,webdev,programmer,portfolio,${this.post.categories.join(',')}`,
      });
      this.meta.updateTag({
        property: `og:url`,
        content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
      });
      this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
      this.meta.updateTag({ property: `og:description`, content: this?.post?.description });
      this.meta.updateTag({
        property: `twitter:url`,
        content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
      });
      this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
      this.meta.updateTag({ property: `twitter:description`, content: this?.post?.description });
    });
    this.commentService
      .getAllCommentsByPost(this.postId, this.commentPage, this.commentPageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(
        (result) => {
          this.blogPostComments = result.data;
          this.commentsCount = result?.pagination?.totalResults;
        },
        (error) => {
          console.log(error);
          if (error.status === 404) {
            this.notFound = true;
            this.title.setTitle('axmouth.dev - Blog Post Not Found');
          }
          this.loading = false;
        },
      );
  }

  onCommentPosted() {
    this.initialiseState();
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
