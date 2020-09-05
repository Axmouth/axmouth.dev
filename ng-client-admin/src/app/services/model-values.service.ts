import { Injectable, OnDestroy, ɵQueryValueType } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { stringify } from 'querystring';
import { Router, NavigationStart } from '@angular/router';
import { RestApiService } from './rest-api.service';
import { throws } from 'assert';

class FieldData {
  subject: BehaviorSubject<any>;
  fieldName: string;
}

@Injectable({
  providedIn: 'root',
})
export class ModelValuesService implements OnDestroy {
  fields: Map<string, FieldData> = new Map<string, FieldData>();

  constructor(private router: Router, private restApiService: RestApiService) {
    router.events.subscribe((event) => {
      if (event instanceof NavigationStart) {
        this.resetFields();
      }
    });
  }

  getValuesObject() {
    const obj: any = {};
    this.fields.forEach((field: FieldData, _: string) => {
      obj[field.fieldName] = field.subject.getValue();
    });

    return obj;
  }

  resetFields() {
    this.fields.forEach((field: FieldData, _: string) => {
      field.subject.complete();
      field.subject.unsubscribe();
    });
    this.fields = new Map<string, FieldData>();
  }

  addField<T>(fieldName: string, initialValue: T): BehaviorSubject<T> {
    const newSubject = new BehaviorSubject<T>(initialValue);
    console.log('added :' + fieldName);
    this.fields.set(fieldName, {
      fieldName,
      subject: newSubject,
    });
    return newSubject;
  }

  sendCreateRequest(url: string): Observable<any> {
    return this.restApiService.create(url, this.getValuesObject(), {});
  }

  sendUpdateRequest(url: string, id: string): Observable<any> {
    return this.restApiService.update(url, id, this.getValuesObject(), {});
  }

  ngOnDestroy() {
    this.resetFields();
  }
}
