import { Component, OnInit } from '@angular/core';
import { AdminModel } from 'src/app/models/definitions/admin-model';
import { AdminModelService } from 'src/app/services/admin-model.service';
import { ActivatedRoute } from '@angular/router';
import { RestApiService } from 'src/app/services/rest-api.service';
import { ModelValuesService } from '../../services/model-values.service';

@Component({
  selector: 'app-view-entity',
  templateUrl: './view-entity.component.html',
  styleUrls: ['./view-entity.component.scss'],
})
export class ViewEntityComponent implements OnInit {
  model: AdminModel;
  modelName: string;
  categoryName: string;
  id: string;
  entitiesList: object[];
  idField: string;
  displayField: string;
  editing = true;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private apiService: RestApiService,
    private modelValueService: ModelValuesService,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.modelName = params.get('modelName');
    this.id = params.get('id');
    this.model = this.modelService.getByModelName(this.modelName);

    this.displayField = this.model.displayField || 'id';
    this.idField = this.model.idField || 'id';

    this.apiService.getAll<{ data: object[] }>(this.model.endpoint, {}).subscribe((result) => {
      this.entitiesList = result.data;
    });
  }

  onSaveClick() {
    console.log(this.model.endpoint);
    console.log(this.id);
    this.modelValueService.sendUpdateRequest(this.model.endpoint, this.id).subscribe((response) => {});
  }

  onRemoveClick() {
    console.log(this.model.endpoint);
    console.log(this.id);
    this.apiService.delete(this.model.endpoint, this.id, {}).subscribe((response) => {});
  }
}
