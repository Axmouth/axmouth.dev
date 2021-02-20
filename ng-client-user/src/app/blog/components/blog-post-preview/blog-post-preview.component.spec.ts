import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { BlogPostPreviewComponent } from './blog-post-preview.component';

describe('BlogPostPreviewComponent', () => {
  let component: BlogPostPreviewComponent;
  let fixture: ComponentFixture<BlogPostPreviewComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ BlogPostPreviewComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostPreviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
