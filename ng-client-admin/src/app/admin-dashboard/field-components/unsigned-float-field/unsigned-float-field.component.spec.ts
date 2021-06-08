import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { UnsignedFloatFieldComponent } from './unsigned-float-field.component';

describe('UnsignedFloatFieldComponent', () => {
  let component: UnsignedFloatFieldComponent;
  let fixture: ComponentFixture<UnsignedFloatFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ UnsignedFloatFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UnsignedFloatFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
