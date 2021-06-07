import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewAdminModelComponent } from './view-admin-model.component';

describe('ViewAdminModelComponent', () => {
  let component: ViewAdminModelComponent;
  let fixture: ComponentFixture<ViewAdminModelComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewAdminModelComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewAdminModelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
