import { Injectable } from '@angular/core';
import { Response } from 'src/app/models/api/response';
import { ProjectTechnology } from '../../models/api/project-technology';
import { apiRoot } from 'src/environments/environment';
import { RestApiService } from 'src/app/shared/services/rest-api.service';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class TechnologyService {
  url = `${apiRoot}/technologies`;
  constructor(private apiService: RestApiService) {}

  getTechnology(id: string): Observable<Response<ProjectTechnology>> {
    return this.apiService.get<Response<ProjectTechnology>>(this.url, id, {});
  }

  getAllTechnologies(page?: number, pageSize?: number): Observable<Response<ProjectTechnology[]>> {
    return this.apiService.getAll<Response<ProjectTechnology[]>>(this.url, { page, pageSize });
  }
}
