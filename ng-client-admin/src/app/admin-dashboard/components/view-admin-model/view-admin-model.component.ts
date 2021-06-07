import { Component, OnInit, OnDestroy } from '@angular/core';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { AdminModelService } from 'src/app/admin-dashboard/services/admin-model.service';
import { ActivatedRoute } from '@angular/router';
import { RestApiService } from '../../services/rest-api.service';
import { Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  selector: 'app-view-admin-model',
  templateUrl: './view-admin-model.component.html',
  styleUrls: ['./view-admin-model.component.scss'],
})
export class ViewAdminModelComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  model: AdminModel;
  modelName: string;
  categoryName: string;
  entitiesList: object[];
  idField: string;
  displayField: string;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private apiService: RestApiService,
    private title: Title,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.modelName = params.get('modelName');
    this.model = this.modelService.getByModelName(this.modelName);

    this.displayField = this.model.displayField || 'id';
    this.idField = this.model.idField || 'id';
    this.title.setTitle(`Model: ${this.modelName} | Axmouth's Website Admin Site`);

    this.apiService
      .getAll<{ data: object[] }>(this.model.endpoint, {})
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.entitiesList = result.data;
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
