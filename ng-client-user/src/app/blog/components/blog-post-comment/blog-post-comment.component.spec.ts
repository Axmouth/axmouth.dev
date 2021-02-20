import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { BlogPostCommentComponent } from './blog-post-comment.component';

describe('BlogPostCommentComponent', () => {
  let component: BlogPostCommentComponent;
  let fixture: ComponentFixture<BlogPostCommentComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ BlogPostCommentComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostCommentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
