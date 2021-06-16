import { Injectable, OnDestroy } from '@angular/core';
import { Observable, Subject } from 'rxjs';
import { Response } from 'src/app/models/api/response';
import { SearchItem } from 'src/app/models/api/search-item';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class SearchService implements OnDestroy {
  url = `${apiRoot}/search`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getAll(
    searchText?: string,
    type?: 'Project' | 'BlogPost' | 'Page' | 'ExternalLink',
    page?: number,
    pageSize?: number,
  ): Observable<Response<SearchItem[]>> {
    return this.apiService
      .getAll<Response<SearchItem[]>>(this.url, { searchText, type, page, pageSize }, true, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
