import { Injectable, OnDestroy } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Router, NavigationStart } from '@angular/router';

class FieldData {
  subject: BehaviorSubject<any>;
  fieldName: string;
  modelName: string;
  categoryName: string;
}

@Injectable({
  providedIn: 'root',
})
export class ModelDraftStorageService implements OnDestroy {
  fields: Map<string, FieldData> = new Map<string, FieldData>();
  constructor(private router: Router) {
    router.events.subscribe((event) => {
      if (event instanceof NavigationStart) {
        this.resetFields();
      }
    });
  }

  resetFields() {
    this.fields.forEach((field: FieldData, _: string) => {
      field.subject.complete();
      field.subject.unsubscribe();
    });
    this.fields = new Map<string, FieldData>();
  }

  addField<T>(categoryName: string, modelName: string, fieldName: string, initialValue: T): BehaviorSubject<T> {
    const newSubject = new BehaviorSubject<T>(initialValue);
    this.fields[categoryName + ':' + modelName + ':' + fieldName] = {
      categoryName,
      fieldName,
      modelName,
      subject: newSubject,
    };
    return newSubject;
  }

  ngOnDestroy() {
    this.resetFields();
  }
}
