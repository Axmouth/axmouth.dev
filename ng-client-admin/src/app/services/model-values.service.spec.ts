import { TestBed } from '@angular/core/testing';

import { ModelValuesService } from './model-values.service';

describe('ModelValuesService', () => {
  let service: ModelValuesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ModelValuesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
