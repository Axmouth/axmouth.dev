import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { BlogPostPreviewComponent } from './blog-post-preview.component';

describe('BlogPostPreviewComponent', () => {
  let component: BlogPostPreviewComponent;
  let fixture: ComponentFixture<BlogPostPreviewComponent>;

  beforeEach(async(() => {
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
