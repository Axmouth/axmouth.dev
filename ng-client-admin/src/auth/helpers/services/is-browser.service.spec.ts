import { TestBed } from '@angular/core/testing';

import { IsBrowserService } from './is-browser.service';

describe('IsBrowserService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: IsBrowserService = TestBed.get(IsBrowserService);
    expect(service).toBeTruthy();
  });
});
