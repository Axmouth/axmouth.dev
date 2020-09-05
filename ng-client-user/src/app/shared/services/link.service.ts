import { Injectable } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { HomePageLink } from '../../models/api/home-page-link';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from './rest-api.service';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class LinkService {
  url = `${apiRoot}/links`;
  constructor(private apiService: RestApiService) {}

  getLink(id: string): Observable<Response<HomePageLink>> {
    return this.apiService.get<Response<HomePageLink>>(this.url, id, {});
  }

  getAllLinks(page?: number, pageSize?: number): Observable<Response<HomePageLink[]>> {
    return this.apiService.getAll<Response<HomePageLink[]>>(this.url, { page, pageSize });
  }
}
