import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { IntroductionSectionComponent } from './introduction-section.component';

describe('IntroductionSectionComponent', () => {
  let component: IntroductionSectionComponent;
  let fixture: ComponentFixture<IntroductionSectionComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ IntroductionSectionComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(IntroductionSectionComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
