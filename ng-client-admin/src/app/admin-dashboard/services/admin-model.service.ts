import { Injectable } from '@angular/core';
import { AdminCategoryService } from './admin-category.service';

@Injectable({
  providedIn: 'root',
})
export class AdminModelService {
  constructor(private categories: AdminCategoryService) {}

  getAll() {
    const cats = this.categories.getAll();
    const models = [];

    for (const cat of cats) {
      for (const model of cat.models) {
        models.push(model);
      }
    }

    return models;
  }

  getByCategoryName(categoryName: string) {
    const cats = this.categories.getAll();

    for (const cat of cats) {
      if (cat.name === categoryName) {
        return cat.models;
      }
    }

    return [];
  }

  getByModelName(modelName: string) {
    const cats = this.categories.getAll();

    for (const cat of cats) {
      for (const model of cat.models) {
        if (model.name === modelName) {
          return model;
        }
      }
    }

    return null;
  }

  getByModelId(modelId: string) {
    const cats = this.categories.getAll();

    for (const cat of cats) {
      for (const model of cat.models) {
        if (model.modelId === modelId) {
          return model;
        }
      }
    }

    return null;
  }
}
