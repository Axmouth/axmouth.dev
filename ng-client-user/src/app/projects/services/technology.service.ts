import { Injectable, OnDestroy } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { ProjectTechnology } from '../../models/api/project-technology';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable, Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class TechnologyService implements OnDestroy {
  url = `${apiRoot}/technologies`;
  ngUnsubscribe = new Subject<void>();

  constructor(private apiService: RestApiService) {}

  getTechnology(id: string): Observable<Response<ProjectTechnology>> {
    return this.apiService.get<Response<ProjectTechnology>>(this.url, id, {}).pipe(takeUntil(this.ngUnsubscribe));
  }

  getAllTechnologies(page?: number, pageSize?: number): Observable<Response<ProjectTechnology[]>> {
    return this.apiService
      .getAll<Response<ProjectTechnology[]>>(this.url, { page, pageSize })
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
