import { Injectable } from '@angular/core';
import { BlogPostCategory } from '../../models/api/blog-post-category';
import { Response } from 'src/app/models/api/response';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class BlogPostCategoryService {
  url = `${apiRoot}/categories`;
  constructor(private apiService: RestApiService) {}

  getCategory(id: string): Observable<Response<BlogPostCategory>> {
    return this.apiService.get<Response<BlogPostCategory>>(this.url, id, {});
  }

  getAllCategories(page?: number, pageSize?: number): Observable<Response<BlogPostCategory[]>> {
    return this.apiService.getAll<Response<BlogPostCategory[]>>(this.url, { page, pageSize });
  }
}
