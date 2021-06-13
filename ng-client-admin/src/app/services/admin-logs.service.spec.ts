import { TestBed } from '@angular/core/testing';

import { AdminLogsService } from './admin-logs.service';

describe('AdminLogsService', () => {
  let service: AdminLogsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AdminLogsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
