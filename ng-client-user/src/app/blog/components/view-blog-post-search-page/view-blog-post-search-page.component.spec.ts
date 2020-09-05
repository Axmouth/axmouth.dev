import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewBlogPostSearchPageComponent } from './view-blog-post-search-page.component';

describe('ViewBlogPostSearchPageComponent', () => {
  let component: ViewBlogPostSearchPageComponent;
  let fixture: ComponentFixture<ViewBlogPostSearchPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewBlogPostSearchPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewBlogPostSearchPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
