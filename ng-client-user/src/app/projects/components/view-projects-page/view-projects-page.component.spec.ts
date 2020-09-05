import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewProjectsPageComponent } from './view-projects-page.component';

describe('ViewProjectsPageComponent', () => {
  let component: ViewProjectsPageComponent;
  let fixture: ComponentFixture<ViewProjectsPageComponent>;

  beforeEach(async(() => {
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
