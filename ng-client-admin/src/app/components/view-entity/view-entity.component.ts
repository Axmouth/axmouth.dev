import { Component, OnInit, OnDestroy } from '@angular/core';
import { AdminModel } from 'src/app/models/definitions/admin-model';
import { AdminModelService } from 'src/app/services/admin-model.service';
import { ActivatedRoute, Router } from '@angular/router';
import { RestApiService } from 'src/app/services/rest-api.service';
import { ModelValuesService } from '../../services/model-values.service';
import { Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';

@Component({
  selector: 'app-view-entity',
  templateUrl: './view-entity.component.html',
  styleUrls: ['./view-entity.component.scss'],
})
export class ViewEntityComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  model: AdminModel;
  modelName: string;
  categoryName: string;
  id: string;
  entity: object;
  idField: string;
  displayField: string;
  editing = true;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private router: Router,
    private apiService: RestApiService,
    private modelValueService: ModelValuesService,
    private title: Title,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.modelName = params.get('modelName');
    this.id = params.get('id');
    this.model = this.modelService.getByModelName(this.modelName);

    this.displayField = this.model.displayField || 'id';
    this.idField = this.model.idField || 'id';

    this.apiService
      .getAll<{ data: object }>(this.model.endpoint, {})
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.entity = result.data;
        this.title.setTitle(`${this.modelName} Entity: ${this.id} | Axmouth's Website Admin Site`);
      });
  }

  onSaveClick() {
    console.log(this.model.endpoint);
    console.log(this.id);
    this.modelValueService
      .sendUpdateRequest(this.model.endpoint, this.id)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((response) => {});
  }

  onRemoveClick() {
    console.log(this.model.endpoint);
    console.log(this.id);
    this.apiService
      .delete(this.model.endpoint, this.id, {})
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((response) => {
        this.router.navigate(['categories', this.categoryName, 'models', this.modelName]);
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
