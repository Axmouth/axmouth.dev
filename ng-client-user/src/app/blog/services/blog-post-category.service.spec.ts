import { TestBed } from '@angular/core/testing';

import { BlogPostCategoryService } from './blog-post-category.service';

describe('BlogPostCategoryService', () => {
  let service: BlogPostCategoryService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(BlogPostCategoryService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
