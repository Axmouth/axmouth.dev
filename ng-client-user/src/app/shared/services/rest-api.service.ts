import { Inject, Injectable, OnDestroy, PLATFORM_ID } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { retry, switchMap, concatMap, map, takeUntil } from 'rxjs/operators';
import { AuthService } from 'src/auth';
import { Observable, of, Subject } from 'rxjs';
import { isPlatformServer } from '@angular/common';
import { apiRoot, apiRootServer } from 'src/environments/environment';

type HttpMethod = 'get' | 'post' | 'put' | 'patch' | 'delete';

function paramsToQuery(params: any) {
  return Object.keys(params)
    .map((key) => {
      if (Array.isArray(params[key])) {
        return params[key]
          .map((value: string | number | boolean) => {
            if (value === undefined || value === null) {
              return '';
            }
            return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`;
          })
          .join('&');
      }
      if (params[key] === undefined || params[key] === null) {
        return '';
      }
      return `${encodeURIComponent(key)}=${encodeURIComponent(params[key])}`;
    })
    .filter((s) => s !== '')
    .join('&');
}

@Injectable({
  providedIn: 'root',
})
export class RestApiService implements OnDestroy {
  static getReqCache = new Map<string, any>();
  ngUnsubscribe = new Subject<void>();

  constructor(
    private http: HttpClient,
    private authService: AuthService,
    @Inject(PLATFORM_ID) private platform: object,
  ) {}

  static getFromCache<T>(url: string, queryParams: any): T {
    const queryString = paramsToQuery(queryParams);
    let newUrl = url;
    if (queryString && queryString.length > 0) {
      newUrl = `${newUrl}?${queryString}`;
    }
    return RestApiService.getReqCache.get(newUrl);
  }

  getAll<T>(baseUrl: string, queryParams: any, cached = false): Observable<T> {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.baseApiRequest<T>(url, queryParams, 'get', undefined, cached);
      }),
    );
  }

  get<T>(baseUrl: string, id: string, queryParams: any, cached = false): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.baseApiRequest<T>(url, queryParams, 'get', undefined, cached);
      }),
    );
  }

  create<T>(baseUrl: string, body: any, queryParams: any): Observable<T> {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.baseApiRequest<T>(url, queryParams, 'post', body);
      }),
    );
  }

  update<T>(baseUrl: string, id: string, body: any, queryParams: any): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.baseApiRequest<T>(url, queryParams, 'put', body);
      }),
    );
  }

  delete<T>(baseUrl: string, id: string, queryParams: any): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.baseApiRequest<T>(url, queryParams, 'delete', undefined);
      }),
    );
  }

  private baseApiRequest<T>(url: string, queryParams: any, method: HttpMethod, body: any, cached = false) {
    if (isPlatformServer(this.platform)) {
      url = url.replace(apiRoot, apiRootServer);
    }
    const headers = new HttpHeaders();
    headers.append('Content-Type', 'application/json');
    const queryString = paramsToQuery(queryParams);
    let newUrl = url;
    if (queryString && queryString.length > 0) {
      newUrl = `${newUrl}?${queryString}`;
    }

    if (cached === true) {
      const cachedReq = RestApiService.getReqCache.get(newUrl);
      if (cachedReq !== undefined) {
        return of(cachedReq);
      }
    }

    return this.http
      .request<T>(method, newUrl, { body, headers, withCredentials: true })
      .pipe(retry(2))
      .pipe(
        map((response) => {
          RestApiService.getReqCache.set(newUrl, response);
          return response;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }

  // TODO Delete Many - Update Many - Create Many
}
