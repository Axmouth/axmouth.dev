import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ProjectsTechnologiesSideWidgetComponent } from './projects-technologies-side-widget.component';

describe('ProjectsTechnologiesSideWidgetComponent', () => {
  let component: ProjectsTechnologiesSideWidgetComponent;
  let fixture: ComponentFixture<ProjectsTechnologiesSideWidgetComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ProjectsTechnologiesSideWidgetComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ProjectsTechnologiesSideWidgetComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
