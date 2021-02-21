import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ProjectPreviewPlaceholderComponent } from './project-preview-placeholder.component';

describe('ProjectPreviewPlaceholderComponent', () => {
  let component: ProjectPreviewPlaceholderComponent;
  let fixture: ComponentFixture<ProjectPreviewPlaceholderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ProjectPreviewPlaceholderComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ProjectPreviewPlaceholderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
