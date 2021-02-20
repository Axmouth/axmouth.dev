import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { LinksSideWidgetComponent } from './links-side-widget.component';

describe('LinksSideWidgetComponent', () => {
  let component: LinksSideWidgetComponent;
  let fixture: ComponentFixture<LinksSideWidgetComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ LinksSideWidgetComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(LinksSideWidgetComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
