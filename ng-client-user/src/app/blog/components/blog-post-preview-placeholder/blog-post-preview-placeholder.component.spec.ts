import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BlogPostPreviewPlaceholderComponent } from './blog-post-preview-placeholder.component';

describe('BlogPostPreviewPlaceholderComponent', () => {
  let component: BlogPostPreviewPlaceholderComponent;
  let fixture: ComponentFixture<BlogPostPreviewPlaceholderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BlogPostPreviewPlaceholderComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostPreviewPlaceholderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
