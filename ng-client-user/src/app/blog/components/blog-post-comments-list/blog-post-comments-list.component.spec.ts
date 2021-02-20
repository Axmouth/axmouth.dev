import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { BlogPostCommentsListComponent } from './blog-post-comments-list.component';

describe('BlogPostCommentsListComponent', () => {
  let component: BlogPostCommentsListComponent;
  let fixture: ComponentFixture<BlogPostCommentsListComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ BlogPostCommentsListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostCommentsListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
