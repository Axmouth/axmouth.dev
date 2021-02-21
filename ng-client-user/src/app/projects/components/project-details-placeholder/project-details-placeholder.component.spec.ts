import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ProjectDetailsPlaceholderComponent } from './project-details-placeholder.component';

describe('ProjectDetailsPlaceholderComponent', () => {
  let component: ProjectDetailsPlaceholderComponent;
  let fixture: ComponentFixture<ProjectDetailsPlaceholderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ProjectDetailsPlaceholderComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ProjectDetailsPlaceholderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
