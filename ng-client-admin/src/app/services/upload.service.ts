import { Injectable } from '@angular/core';
import { HttpHeaders, HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthService } from 'src/auth/services/auth.service';
import { concatMap } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class UploadService {
  constructor(private http: HttpClient, private authService: AuthService) {}

  uploadFile<T>(url: string, name: string, file: File): Observable<T> {
    const myFormData = new FormData();
    const headers = new HttpHeaders();
    headers.append('Content-Type', 'multipart/form-data');
    headers.append('Accept', 'application/json');
    myFormData.append('image', file);
    return this.authService.isAuthenticatedOrRefresh().pipe(
      concatMap(() => {
        return this.http.post<T>(url, myFormData, {
          headers,
          withCredentials: true,
        });
      }),
    );
  }
}
