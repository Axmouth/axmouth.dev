import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { LinkFieldComponent } from './link-field.component';

describe('LinkFieldComponent', () => {
  let component: LinkFieldComponent;
  let fixture: ComponentFixture<LinkFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ LinkFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(LinkFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
