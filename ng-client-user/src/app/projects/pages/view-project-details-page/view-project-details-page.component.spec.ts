import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewProjectDetailsPageComponent } from './view-project-details-page.component';

describe('ViewProjectDetailsPageComponent', () => {
  let component: ViewProjectDetailsPageComponent;
  let fixture: ComponentFixture<ViewProjectDetailsPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewProjectDetailsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewProjectDetailsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
