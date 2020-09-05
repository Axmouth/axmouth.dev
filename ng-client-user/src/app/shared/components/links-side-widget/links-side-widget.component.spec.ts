import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { LinksSideWidgetComponent } from './links-side-widget.component';

describe('LinksSideWidgetComponent', () => {
  let component: LinksSideWidgetComponent;
  let fixture: ComponentFixture<LinksSideWidgetComponent>;

  beforeEach(async(() => {
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
