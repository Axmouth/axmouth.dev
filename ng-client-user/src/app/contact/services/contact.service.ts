import { Injectable } from '@angular/core';
import { RestApiService } from '../../shared/services/rest-api.service';
import { apiRoot } from 'src/environments/environment';
import { Observable, of } from 'rxjs';
import { Response } from '../../models/api/response';
import { catchError } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class ContactService {
  url = `${apiRoot}/contact/contact-email`;

  constructor(private apiService: RestApiService) {}

  sendContactEmail(
    subject: string,
    fromEmail: string,
    body: string,
    captchaToken: string,
  ): Observable<Response<number | undefined>> {
    return this.apiService
      .create<Response<number | undefined>>(this.url, { subject, fromEmail, body, captchaToken }, {})
      .pipe(
        catchError((result) => {
          console.log('catchError');
          console.log(result);
          return of(result.error as Response<number | null>);
        }),
      );
  }
}
