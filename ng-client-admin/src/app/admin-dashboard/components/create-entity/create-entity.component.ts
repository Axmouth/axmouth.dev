import { Component, OnInit } from '@angular/core';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { AdminModelService } from 'src/app/admin-dashboard/services/admin-model.service';
import { ActivatedRoute, Router } from '@angular/router';
import { ModelValuesService } from 'src/app/admin-dashboard/services/model-values.service';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-create-entity',
  templateUrl: './create-entity.component.html',
  styleUrls: ['./create-entity.component.scss'],
})
export class CreateEntityComponent implements OnInit {
  model: AdminModel;
  modelName: string;
  categoryName: string;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private router: Router,
    private modelValuesService: ModelValuesService,
    private title: Title,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.modelName = params.get('modelName');
    this.categoryName = params.get('categoryName');
    this.model = this.modelService.getByModelName(this.modelName);
    this.title.setTitle(`Create a ${this.modelName} | Axmouth's Website Admin Site`);
  }

  onSaveClick() {
    this.modelValuesService.sendCreateRequest(this.model.endpoint).subscribe((response) => {
      this.router.navigate(['categories', this.categoryName, 'models', this.modelName]);
    });
  }
}
