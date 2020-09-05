import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewBlogPostsCategoryPageComponent } from './view-blog-posts-category-page.component';

describe('ViewBlogPostsCategoryPageComponent', () => {
  let component: ViewBlogPostsCategoryPageComponent;
  let fixture: ComponentFixture<ViewBlogPostsCategoryPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewBlogPostsCategoryPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewBlogPostsCategoryPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
