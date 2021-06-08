import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/admin-dashboard/definitions/admin-model-field';
import { ModelValuesService } from 'src/app/admin-dashboard/services/model-values.service';
import { BehaviorSubject } from 'rxjs';

@Component({
  selector: 'app-title-field',
  templateUrl: './title-field.component.html',
  styleUrls: ['./title-field.component.scss'],
})
export class TitleFieldComponent implements OnInit {
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;
  subject: BehaviorSubject<string>;

  constructor(private modelValuesService: ModelValuesService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, undefined);
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  onChange(text: string) {
    console.log(text);
    this.subject.next(text);
    this.contentChange.next(text);
  }
}
