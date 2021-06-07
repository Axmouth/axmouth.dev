import { Injectable } from '@angular/core';
import { AdminCategory } from 'src/app/admin-dashboard/definitions/admin-category';
import { categoryList } from 'src/app/admin-dashboard/definitions/admin-config';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { FieldType } from 'src/app/admin-dashboard/definitions/admin-model-field';

@Injectable({
  providedIn: 'root',
})
export class AdminCategoryService {
  constructor() {}

  getAll(): AdminCategory[] {
    return categoryList;
  }

  get(categoryName: string): AdminCategory {
    let i: number;
    for (i = 0; i < categoryList.length; i++) {
      if (categoryList[i].name === categoryName) {
        return categoryList[i];
      }
    }

    return undefined;
  }
}
