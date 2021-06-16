import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SearchSideWidgetComponent } from './search-side-widget.component';

describe('SearchSideWidgetComponent', () => {
  let component: SearchSideWidgetComponent;
  let fixture: ComponentFixture<SearchSideWidgetComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SearchSideWidgetComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SearchSideWidgetComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
