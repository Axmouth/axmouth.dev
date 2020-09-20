import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { ModelValuesService } from 'src/app/services/model-values.service';

@Component({
  selector: 'app-text-field',
  templateUrl: './text-field.component.html',
  styleUrls: ['./text-field.component.scss'],
})
export class TextFieldComponent implements OnInit {
  subject: BehaviorSubject<string>;
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;

  constructor(private modelValuesService: ModelValuesService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, null);
    if (this.content === undefined) {
      this.content = null;
    }
    if (this.content) {
      this.subject.next(this.content);
    }
  }
}
