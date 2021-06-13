import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { NameListFieldComponent } from './name-list-field.component';

describe('NameListFieldComponent', () => {
  let component: NameListFieldComponent;
  let fixture: ComponentFixture<NameListFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ NameListFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(NameListFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
