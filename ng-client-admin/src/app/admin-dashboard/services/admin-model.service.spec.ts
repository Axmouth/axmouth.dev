import { TestBed } from '@angular/core/testing';

import { AdminModelService } from './admin-model.service';

describe('AdminModelService', () => {
  let service: AdminModelService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AdminModelService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
