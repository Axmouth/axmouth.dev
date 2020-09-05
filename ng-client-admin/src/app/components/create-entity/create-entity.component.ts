import { Component, OnInit } from '@angular/core';
import { AdminModel } from 'src/app/models/definitions/admin-model';
import { AdminModelService } from 'src/app/services/admin-model.service';
import { ActivatedRoute } from '@angular/router';
import { ModelValuesService } from '../../services/model-values.service';

@Component({
  selector: 'app-create-entity',
  templateUrl: './create-entity.component.html',
  styleUrls: ['./create-entity.component.scss'],
})
export class CreateEntityComponent implements OnInit {
  model: AdminModel;
  modelName: string;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private modelValuesService: ModelValuesService,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.modelName = params.get('modelName');
    this.model = this.modelService.getByModelName(this.modelName);
  }

  onSaveClick() {
    this.modelValuesService.sendCreateRequest(this.model.endpoint).subscribe((response) => {});
  }
}
