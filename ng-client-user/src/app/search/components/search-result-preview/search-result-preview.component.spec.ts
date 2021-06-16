import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SearchResultPreviewComponent } from './search-result-preview.component';

describe('SearchResultPreviewComponent', () => {
  let component: SearchResultPreviewComponent;
  let fixture: ComponentFixture<SearchResultPreviewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ SearchResultPreviewComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SearchResultPreviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
