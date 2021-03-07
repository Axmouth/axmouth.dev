import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewProjectsPageComponent } from './view-projects-page.component';

describe('ViewProjectsPageComponent', () => {
  let component: ViewProjectsPageComponent;
  let fixture: ComponentFixture<ViewProjectsPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewProjectsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewProjectsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
