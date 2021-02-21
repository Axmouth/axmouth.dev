import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BlogPostDetailsPlaceholderComponent } from './blog-post-details-placeholder.component';

describe('BlogPostDetailsPlaceholderComponent', () => {
  let component: BlogPostDetailsPlaceholderComponent;
  let fixture: ComponentFixture<BlogPostDetailsPlaceholderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BlogPostDetailsPlaceholderComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogPostDetailsPlaceholderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
