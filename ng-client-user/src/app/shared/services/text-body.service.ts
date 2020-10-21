import { Injectable, OnDestroy } from '@angular/core';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from './rest-api.service';
import { Observable, Subject } from 'rxjs';
import { Response } from 'src/app/models/api/response';
import { TextBody } from '../../models/api/text-body';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class TextBodyService implements OnDestroy {
  url = `${apiRoot}/text-bodies`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getTextBody(slug: string): Observable<Response<TextBody>> {
    return this.apiService.get<Response<TextBody>>(this.url, slug, {}, true, false).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllTextBodies(page?: number, pageSize?: number): Observable<Response<TextBody[]>> {
    return this.apiService
      .getAll<Response<TextBody[]>>(this.url, { page, pageSize }, true, false)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
