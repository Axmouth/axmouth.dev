import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { AuthService } from '../../auth/services/auth.service';
import { switchMap } from 'rxjs/operators';

type HttpMethod = 'get' | 'post' | 'put' | 'patch' | 'delete';

function paramsToQuery(params: any) {
  return Object.keys(params)
    .map((key) => {
      if (Array.isArray(params[key])) {
        return params[key]
          .map((value: string | number | boolean) => {
            return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`;
          })
          .join('&');
      }
      return `${encodeURIComponent(key)}=${encodeURIComponent(params[key])}`;
    })
    .join('&');
}

function baseApiRequest<T>(http: HttpClient, url: string, queryParams: any, method: HttpMethod, body: any) {
  const headers = new HttpHeaders();
  headers.append('Content-Type', 'application/json');
  return http.request<T>(method, url + '?' + paramsToQuery(queryParams), { body, headers, withCredentials: true });
}
@Injectable({
  providedIn: 'root',
})
export class RestApiService {
  constructor(private http: HttpClient, private authService: AuthService) {}

  getAll<T>(baseUrl: string, queryParams: any) {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh(undefined).pipe(
      switchMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'get', undefined);
      }),
    );
  }

  get<T>(baseUrl: string, id: string, queryParams: any) {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh(undefined).pipe(
      switchMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'get', undefined);
      }),
    );
  }

  create<T>(baseUrl: string, body: any, queryParams: any) {
    const url = `${baseUrl}`;

    return this.authService.isAuthenticatedOrRefresh(undefined).pipe(
      switchMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'post', body);
      }),
    );
  }

  update<T>(baseUrl: string, id: string, body: any, queryParams: any) {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh(undefined).pipe(
      switchMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'put', body);
      }),
    );
  }

  delete<T>(baseUrl: string, id: string, queryParams: any) {
    const url = `${baseUrl}/${id}`;

    return this.authService.isAuthenticatedOrRefresh(undefined).pipe(
      switchMap(() => {
        return baseApiRequest<T>(this.http, url, queryParams, 'delete', undefined);
      }),
    );
  }

  // TODO Delete Many - Update Many - Create Many
}
