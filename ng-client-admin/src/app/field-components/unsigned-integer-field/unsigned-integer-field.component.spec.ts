import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UnsignedIntegerFieldComponent } from './unsigned-integer-field.component';

describe('UnsignedIntegerFieldComponent', () => {
  let component: UnsignedIntegerFieldComponent;
  let fixture: ComponentFixture<UnsignedIntegerFieldComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UnsignedIntegerFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UnsignedIntegerFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
