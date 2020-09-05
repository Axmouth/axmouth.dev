import { Injectable } from '@angular/core';
import { RestApiService } from '../../shared/services/rest-api.service';
import { apiRoot } from 'src/environments/environment';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ContactService {
  url = `${apiRoot}/contact/contact-email`;

  constructor(private apiService: RestApiService) {}

  sendContactEmail(subject: string, fromEmail: string, body: string, captchaToken: string): Observable<any> {
    return this.apiService.create(this.url, { subject, fromEmail, body, captchaToken }, {});
  }
}
