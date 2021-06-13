import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { AdminModelComponent } from './admin-model.component';

describe('AdminModelComponent', () => {
  let component: AdminModelComponent;
  let fixture: ComponentFixture<AdminModelComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ AdminModelComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AdminModelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
