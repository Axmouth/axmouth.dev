import { Injectable, OnDestroy } from '@angular/core';
import { BlogPostCategory } from '../../models/api/blog-post-category';
import { Response } from 'src/app/models/api/response';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable, Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class BlogPostCategoryService implements OnDestroy {
  url = `${apiRoot}/categories`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getCategory(id: string): Observable<Response<BlogPostCategory>> {
    return this.apiService.get<Response<BlogPostCategory>>(this.url, id, {}, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllCategories(page?: number, pageSize?: number): Observable<Response<BlogPostCategory[]>> {
    return this.apiService
      .getAll<Response<BlogPostCategory[]>>(this.url, { page, pageSize }, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
