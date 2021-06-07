import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/admin-dashboard/definitions/admin-model-field';
import { BehaviorSubject } from 'rxjs';
import { ModelValuesService } from 'src/app/admin-dashboard/services/model-values.service';

@Component({
  selector: 'app-enum-field',
  templateUrl: './enum-field.component.html',
  styleUrls: ['./enum-field.component.scss'],
})
export class EnumFieldComponent implements OnInit {
  @Input()
  content: boolean;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;
  subject: BehaviorSubject<boolean>;

  constructor(private modelValuesService: ModelValuesService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, undefined);
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  onChange(checked: boolean) {
    console.log(checked);
    this.subject.next(checked);
  }
}
