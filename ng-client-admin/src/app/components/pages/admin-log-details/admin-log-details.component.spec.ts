import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminLogDetailsComponent } from './admin-log-details.component';

describe('AdminLogDetailsComponent', () => {
  let component: AdminLogDetailsComponent;
  let fixture: ComponentFixture<AdminLogDetailsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AdminLogDetailsComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(AdminLogDetailsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
