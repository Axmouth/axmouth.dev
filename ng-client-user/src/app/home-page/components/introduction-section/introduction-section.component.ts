import { Component, OnInit } from '@angular/core';
import { TextBodyService } from '../../../shared/services/text-body.service';

@Component({
  selector: 'app-introduction-section',
  templateUrl: './introduction-section.component.html',
  styleUrls: ['./introduction-section.component.scss'],
})
export class IntroductionSectionComponent implements OnInit {
  introduction: string;

  constructor(private textBodyService: TextBodyService) {}

  ngOnInit(): void {
    this.textBodyService.getTextBody('introduction').subscribe((result) => {
      this.introduction = JSON.parse(result.data.body);
    });
  }
}
