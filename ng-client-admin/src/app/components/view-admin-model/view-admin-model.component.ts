import { Component, OnInit } from '@angular/core';
import { AdminModel } from '../../models/definitions/admin-model';
import { AdminModelService } from '../../services/admin-model.service';
import { ActivatedRoute } from '@angular/router';
import { RestApiService } from '../../services/rest-api.service';

@Component({
  selector: 'app-view-admin-model',
  templateUrl: './view-admin-model.component.html',
  styleUrls: ['./view-admin-model.component.scss'],
})
export class ViewAdminModelComponent implements OnInit {
  model: AdminModel;
  modelName: string;
  categoryName: string;
  typesOfShoes: string[] = ['Boots', 'Clogs', 'Loafers', 'Moccasins', 'Sneakers'];
  entitiesList: object[];
  idField: string;
  displayField: string;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private apiService: RestApiService,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.modelName = params.get('modelName');
    this.model = this.modelService.getByModelName(this.modelName);

    this.displayField = this.model.displayField || 'id';
    this.idField = this.model.idField || 'id';

    this.apiService.getAll<{ data: object[] }>(this.model.endpoint, {}).subscribe((result) => {
      this.entitiesList = result.data;
    });
  }
}
