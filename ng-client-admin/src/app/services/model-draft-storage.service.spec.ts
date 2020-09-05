import { TestBed } from '@angular/core/testing';

import { ModelDraftStorageService } from './model-draft-storage.service';

describe('ModelDraftStorageService', () => {
  let service: ModelDraftStorageService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ModelDraftStorageService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
