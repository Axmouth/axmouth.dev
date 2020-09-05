import { Component, OnInit, Input } from '@angular/core';
import { AdminModel } from '../../models/definitions/admin-model';
import { RestApiService } from '../../services/rest-api.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-admin-model',
  templateUrl: './admin-model.component.html',
  styleUrls: ['./admin-model.component.scss'],
})
export class AdminModelComponent implements OnInit {
  @Input()
  model: AdminModel;
  @Input()
  editing: boolean;
  @Input()
  creating: boolean;
  entity: object;
  categoryName: string;
  modelName: string;
  id: string;
  loaded = false;

  constructor(private route: ActivatedRoute, private apiService: RestApiService) {}

  ngOnInit(): void {
    if (this.editing) {
      const params = this.route.snapshot.paramMap;
      this.categoryName = params.get('categoryName');
      this.modelName = params.get('modelName');
      this.id = params.get('id');

      this.apiService.get<{ data: object }>(this.model.endpoint, this.id, {}).subscribe((result) => {
        this.entity = result.data;
        this.loaded = true;
      });
    }
  }
}
