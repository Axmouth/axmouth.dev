import { Injectable } from '@angular/core';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from './rest-api.service';
import { Observable } from 'rxjs';
import { Response } from 'src/app/models/api/response';
import { TextBody } from '../../models/api/text-body';

@Injectable({
  providedIn: 'root',
})
export class TextBodyService {
  url = `${apiRoot}/text-bodies`;
  constructor(private apiService: RestApiService) {}

  getTextBody(slug: string): Observable<Response<TextBody>> {
    return this.apiService.get<Response<TextBody>>(this.url, slug, {});
  }

  getAllTextBodies(page?: number, pageSize?: number): Observable<Response<TextBody[]>> {
    return this.apiService.getAll<Response<TextBody[]>>(this.url, { page, pageSize });
  }
}
