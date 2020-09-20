import { Injectable, OnDestroy } from '@angular/core';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable, Subject } from 'rxjs';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';
import { Response } from 'src/app/models/api/response';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class BlogPostCommentService implements OnDestroy {
  url = `${apiRoot}/blog-post-comments`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getComment(id: string): Observable<Response<BlogPostComment>> {
    return this.apiService.get<Response<BlogPostComment>>(this.url, id, {}).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllComments(page?: number, pageSize?: number): Observable<Response<BlogPostComment[]>> {
    return this.apiService
      .getAll<Response<BlogPostComment[]>>(this.url, { page, pageSize })
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllCommentsByPost(post: string, page?: number, pageSize?: number): Observable<Response<BlogPostComment[]>> {
    return this.apiService
      .getAll<Response<BlogPostComment[]>>(this.url, { page, pageSize, post })
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  createComment(body: string, postId: number): Observable<Response<BlogPostComment>> {
    return this.apiService
      .create<Response<BlogPostComment>>(this.url, { body, postId }, {})
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
