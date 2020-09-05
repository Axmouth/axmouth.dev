import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { BlogCategorySideWidgetComponent } from './blog-category-side-widget.component';

describe('BlogCategorySideWidgetComponent', () => {
  let component: BlogCategorySideWidgetComponent;
  let fixture: ComponentFixture<BlogCategorySideWidgetComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ BlogCategorySideWidgetComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlogCategorySideWidgetComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
