import { Injectable } from '@angular/core';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable } from 'rxjs';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';
import { Response } from 'src/app/models/api/response';

@Injectable({
  providedIn: 'root',
})
export class BlogPostCommentService {
  url = `${apiRoot}/blog-post-comments`;
  constructor(private apiService: RestApiService) {}

  getComment(id: string): Observable<Response<BlogPostComment>> {
    return this.apiService.get<Response<BlogPostComment>>(this.url, id, {});
  }

  getAllComments(page?: number, pageSize?: number): Observable<Response<BlogPostComment[]>> {
    return this.apiService.getAll<Response<BlogPostComment[]>>(this.url, { page, pageSize });
  }

  getAllCommentsByPost(post: string, page?: number, pageSize?: number): Observable<Response<BlogPostComment[]>> {
    return this.apiService.getAll<Response<BlogPostComment[]>>(this.url, { page, pageSize, post });
  }

  createComment(body: string, postId: number): Observable<Response<BlogPostComment>> {
    return this.apiService.create<Response<BlogPostComment>>(this.url, { body, postId }, {});
  }
}
