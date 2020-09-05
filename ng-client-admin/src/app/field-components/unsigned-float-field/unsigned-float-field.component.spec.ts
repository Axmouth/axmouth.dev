import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UnsignedFloatFieldComponent } from './unsigned-float-field.component';

describe('UnsignedFloatFieldComponent', () => {
  let component: UnsignedFloatFieldComponent;
  let fixture: ComponentFixture<UnsignedFloatFieldComponent>;

  beforeEach(async(() => {
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
