import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BlogPostCommentPlaceholderComponent } from './blog-post-comment-placeholder.component';

describe('BlogPostCommentPlaceholderComponent', () => {
  let component: BlogPostCommentPlaceholderComponent;
  let fixture: ComponentFixture<BlogPostCommentPlaceholderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BlogPostCommentPlaceholderComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostCommentPlaceholderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
