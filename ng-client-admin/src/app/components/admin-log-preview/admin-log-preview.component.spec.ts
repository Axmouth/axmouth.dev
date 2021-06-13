import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminLogPreviewComponent } from './admin-log-preview.component';

describe('AdminLogPreviewComponent', () => {
  let component: AdminLogPreviewComponent;
  let fixture: ComponentFixture<AdminLogPreviewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AdminLogPreviewComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(AdminLogPreviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
