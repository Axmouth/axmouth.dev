import { Injectable, OnDestroy } from '@angular/core';
import { RestApiService } from '../../shared/services/rest-api.service';
import { apiRoot } from 'src/environments/environment';
import { BlogPost } from 'src/app/models/api/blog-post';
import { Observable, Subject } from 'rxjs';
import { Response } from 'src/app/models/api/response';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class BlogPostService implements OnDestroy {
  url = `${apiRoot}/blog-posts`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getPost(id: string, query: object): Observable<Response<BlogPost>> {
    return this.apiService.get<Response<BlogPost>>(this.url, id, query, true, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllPosts(query: object): Observable<Response<BlogPost[]>> {
    return this.apiService
      .getAll<Response<BlogPost[]>>(this.url, query, true, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllPostsByCategory(categoryName: string, page?: number, pageSize?: number): Observable<Response<BlogPost[]>> {
    return this.apiService
      .getAll<Response<BlogPost[]>>(this.url, { page, pageSize, categoryName }, true, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
