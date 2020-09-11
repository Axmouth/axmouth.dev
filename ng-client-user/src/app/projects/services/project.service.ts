import { Injectable } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { Project } from '../../models/api/project';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable, of } from 'rxjs';
import { Router } from '@angular/router';

@Injectable({
  providedIn: 'root',
})
export class ProjectService {
  url = `${apiRoot}/projects`;
  constructor(private apiService: RestApiService, private router: Router) {}

  getProject(id: string): Observable<Response<Project>> {
    return this.apiService.get<Response<Project>>(this.url, id, {});
  }

  getAllProjects(page?: number, pageSize?: number): Observable<Response<Project[]>> {
    return this.apiService.getAll<Response<Project[]>>(this.url, { page, pageSize });
  }

  getAllProjectsByTechnology(category: string, page?: number, pageSize?: number): Observable<Response<Project[]>> {
    return this.apiService.getAll<Response<Project[]>>(this.url, { page, pageSize, category });
  }
}
