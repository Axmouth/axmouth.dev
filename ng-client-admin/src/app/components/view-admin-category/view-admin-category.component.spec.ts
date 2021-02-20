import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewAdminCategoryComponent } from './view-admin-category.component';

describe('ViewAdminCategoryComponent', () => {
  let component: ViewAdminCategoryComponent;
  let fixture: ComponentFixture<ViewAdminCategoryComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewAdminCategoryComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewAdminCategoryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
