import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminModelComponent } from './admin-model.component';

describe('AdminModelComponent', () => {
  let component: AdminModelComponent;
  let fixture: ComponentFixture<AdminModelComponent>;

  beforeEach(async(() => {
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
