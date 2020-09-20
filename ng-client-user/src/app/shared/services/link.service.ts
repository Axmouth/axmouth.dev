import { Injectable, OnDestroy } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { HomePageLink } from '../../models/api/home-page-link';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from './rest-api.service';
import { Observable, Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class LinkService implements OnDestroy {
  static url = `${apiRoot}/links`;
  ngUnsubscribe = new Subject<void>();

  static getAllLinksFromCache(page?: number, pageSize?: number): Response<HomePageLink[]> {
    return RestApiService.getFromCache<Response<HomePageLink[]>>(LinkService.url, { page, pageSize });
  }

  constructor(private apiService: RestApiService) {}

  getLink(id: string): Observable<Response<HomePageLink>> {
    return this.apiService.get<Response<HomePageLink>>(LinkService.url, id, {}).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllLinks(page?: number, pageSize?: number): Observable<Response<HomePageLink[]>> {
    return this.apiService
      .getAll<Response<HomePageLink[]>>(LinkService.url, { page, pageSize }, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
