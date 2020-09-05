import { Injectable } from '@angular/core';
import { RestApiService } from '../../shared/services/rest-api.service';
import { apiRoot } from 'src/environments/environment';
import { BlogPost } from 'src/app/models/api/blog-post';
import { Observable } from 'rxjs';
import { Response } from 'src/app/models/api/response';

@Injectable({
  providedIn: 'root',
})
export class BlogPostService {
  url = `${apiRoot}/blog-posts`;
  constructor(private apiService: RestApiService) {}

  getPost(id: string): Observable<Response<BlogPost>> {
    return this.apiService.get<Response<BlogPost>>(this.url, id, {});
  }

  getAllPosts(page?: number, pageSize?: number): Observable<Response<BlogPost[]>> {
    return this.apiService.getAll<Response<BlogPost[]>>(this.url, { page, pageSize });
  }

  getAllPostsByCategory(categoryName: string, page?: number, pageSize?: number): Observable<Response<BlogPost[]>> {
    return this.apiService.getAll<Response<BlogPost[]>>(this.url, { page, pageSize, categoryName });
  }
}
