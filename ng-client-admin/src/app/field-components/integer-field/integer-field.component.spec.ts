import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { IntegerFieldComponent } from './integer-field.component';

describe('IntegerFieldComponent', () => {
  let component: IntegerFieldComponent;
  let fixture: ComponentFixture<IntegerFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ IntegerFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(IntegerFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
