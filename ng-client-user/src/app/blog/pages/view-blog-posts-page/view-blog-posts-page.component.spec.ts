import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewBlogPostsPageComponent } from './view-blog-posts-page.component';

describe('ViewBlogPostsPageComponent', () => {
  let component: ViewBlogPostsPageComponent;
  let fixture: ComponentFixture<ViewBlogPostsPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewBlogPostsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewBlogPostsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
