import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { EnumFieldComponent } from './enum-field.component';

describe('EnumFieldComponent', () => {
  let component: EnumFieldComponent;
  let fixture: ComponentFixture<EnumFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ EnumFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EnumFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
