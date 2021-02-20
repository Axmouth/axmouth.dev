import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ProjectsMainComponent } from './projects-main.component';

describe('ProjectsMainComponent', () => {
  let component: ProjectsMainComponent;
  let fixture: ComponentFixture<ProjectsMainComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ProjectsMainComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ProjectsMainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
