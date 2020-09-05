import { TestBed } from '@angular/core/testing';

import { BlogPostCommentService } from './blog-post-comment.service';

describe('BlogPostCommentService', () => {
  let service: BlogPostCommentService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(BlogPostCommentService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
