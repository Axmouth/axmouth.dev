import { TestBed } from '@angular/core/testing';

import { TextBodyService } from './text-body.service';

describe('TextBodyService', () => {
  let service: TextBodyService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(TextBodyService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
