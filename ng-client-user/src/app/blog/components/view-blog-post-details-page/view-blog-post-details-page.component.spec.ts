import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewBlogPostDetailsPageComponent } from './view-blog-post-details-page.component';

describe('ViewBlogPostDetailsPageComponent', () => {
  let component: ViewBlogPostDetailsPageComponent;
  let fixture: ComponentFixture<ViewBlogPostDetailsPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewBlogPostDetailsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewBlogPostDetailsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
