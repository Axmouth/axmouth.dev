import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { BlogSearchSideWidgetComponent } from './blog-search-side-widget.component';

describe('BlogSearchSideWidgetComponent', () => {
  let component: BlogSearchSideWidgetComponent;
  let fixture: ComponentFixture<BlogSearchSideWidgetComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ BlogSearchSideWidgetComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogSearchSideWidgetComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
