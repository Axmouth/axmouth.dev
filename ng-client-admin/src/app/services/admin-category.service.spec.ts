import { TestBed } from '@angular/core/testing';

import { AdminCategoryService } from './admin-category.service';

describe('AdminCategoryService', () => {
  let service: AdminCategoryService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AdminCategoryService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
