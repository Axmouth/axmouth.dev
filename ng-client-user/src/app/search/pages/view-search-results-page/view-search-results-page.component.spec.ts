import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewSearchResultsPageComponent } from './view-search-results-page.component';

describe('ViewSearchResultsPageComponent', () => {
  let component: ViewSearchResultsPageComponent;
  let fixture: ComponentFixture<ViewSearchResultsPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ViewSearchResultsPageComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewSearchResultsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
