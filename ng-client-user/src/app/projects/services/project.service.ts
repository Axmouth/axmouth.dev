import { Injectable, OnDestroy } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { Project } from '../../models/api/project';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable, of, Subject } from 'rxjs';
import { Router } from '@angular/router';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class ProjectService implements OnDestroy {
  url = `${apiRoot}/projects`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService, private router: Router) {}

  getProject(id: string, query: object): Observable<Response<Project>> {
    return this.apiService.get<Response<Project>>(this.url, id, query, true, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllProjects(query: object = {}): Observable<Response<Project[]>> {
    return this.apiService.getAll<Response<Project[]>>(this.url, query, true, true).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllProjectsByTechnology(
    technologyName: string,
    page?: number,
    pageSize?: number,
  ): Observable<Response<Project[]>> {
    return this.apiService
      .getAll<Response<Project[]>>(this.url, { page, pageSize, technologyName }, true, true)
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
