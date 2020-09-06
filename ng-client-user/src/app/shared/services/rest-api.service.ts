import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { retry, switchMap, concatMap } from 'rxjs/operators';
import { AuthService } from 'src/auth';
import { Observable } from 'rxjs';

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

function baseApiRequest<T>(http: HttpClient, url: string, queryParams: any, method: HttpMethod, body: any) {
  const headers = new HttpHeaders();
  headers.append('Content-Type', 'application/json');
  const queryString = paramsToQuery(queryParams);
  let newUrl = url;
  if (queryString && queryString.length > 0) {
    newUrl = `${newUrl}?${queryString}`;
  }
  return http
    .request<T>(method, newUrl, { body, headers, withCredentials: true })
    .pipe(retry(2));
}

@Injectable({
  providedIn: 'root',
})
export class RestApiService {
  constructor(private http: HttpClient, private authService: AuthService) {}

  getAll<T>(baseUrl: string, queryParams: any): Observable<T> {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'get', undefined);
      }),
    );
  }

  get<T>(baseUrl: string, id: string, queryParams: any): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'get', undefined);
      }),
    );
  }

  create<T>(baseUrl: string, body: any, queryParams: any): Observable<T> {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'post', body);
      }),
    );
  }

  update<T>(baseUrl: string, id: string, body: any, queryParams: any): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'put', body);
      }),
    );
  }

  delete<T>(baseUrl: string, id: string, queryParams: any): Observable<T> {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'delete', undefined);
      }),
    );
  }

  // TODO Delete Many - Update Many - Create Many
}
