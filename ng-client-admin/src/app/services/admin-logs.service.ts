import { Injectable, OnDestroy } from '@angular/core';
import { RestApiService } from '../admin-dashboard/services/rest-api.service';
import { apiRoot } from 'src/environments/environment';
import { Observable, of, Subject } from 'rxjs';
import { Router } from '@angular/router';
import { takeUntil } from 'rxjs/operators';
import { Response } from 'src/app/models/api/response';
import { AdminLog } from '../models/api/admin-log';

@Injectable({
  providedIn: 'root',
})
export class AdminLogsService implements OnDestroy {
  url = `${apiRoot}/admin-logs`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService, private router: Router) {}

  get(id: string, query: object): Observable<Response<AdminLog>> {
    return this.apiService.get<Response<AdminLog>>(this.url, id, query, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAll(query: object = {}): Observable<Response<AdminLog[]>> {
    return this.apiService.getAll<Response<AdminLog[]>>(this.url, query, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
