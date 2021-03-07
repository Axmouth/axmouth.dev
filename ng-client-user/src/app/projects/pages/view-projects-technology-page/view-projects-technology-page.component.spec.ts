import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewProjectsTechnologyPageComponent } from './view-projects-technology-page.component';

describe('ViewProjectsTechnologyPageComponent', () => {
  let component: ViewProjectsTechnologyPageComponent;
  let fixture: ComponentFixture<ViewProjectsTechnologyPageComponent>;

  beforeEach(waitForAsync(() => {
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
