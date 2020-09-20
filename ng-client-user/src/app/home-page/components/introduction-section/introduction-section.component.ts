import { Component, OnInit, OnDestroy } from '@angular/core';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { TextBodyService } from '../../../shared/services/text-body.service';

@Component({
  selector: 'app-introduction-section',
  templateUrl: './introduction-section.component.html',
  styleUrls: ['./introduction-section.component.scss'],
})
export class IntroductionSectionComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  introduction: any;

  constructor(private textBodyService: TextBodyService) {}

  ngOnInit(): void {
    this.textBodyService
      .getTextBody('introduction')
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.introduction = JSON.parse(result.data.body);
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
