import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewProjectsTechnologyPageComponent } from './view-projects-technology-page.component';

describe('ViewProjectsTechnologyPageComponent', () => {
  let component: ViewProjectsTechnologyPageComponent;
  let fixture: ComponentFixture<ViewProjectsTechnologyPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewProjectsTechnologyPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewProjectsTechnologyPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
