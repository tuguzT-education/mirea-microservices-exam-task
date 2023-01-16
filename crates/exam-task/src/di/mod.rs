use std::sync::Arc;

use shaku::{module, ModuleBuilder};

use self::{
    data_source::DatabaseUrl,
    repository::{IdRepository, TaskRepository},
};

mod data_source;
mod repository;
mod schedule;
mod use_case;

module! {
    pub AppModule {
        components = [
            data_source::DatabaseUrl,
            data_source::ClientComponent,
            data_source::TaskDataSourceComponent,
            repository::TaskRepositoryComponent,
            repository::IdRepositoryComponent,
            use_case::CreateTaskUseCaseComponent,
            use_case::DeleteTaskUseCaseComponent,
            use_case::ReadTaskUseCaseComponent,
            use_case::UpdateTaskUseCaseComponent,
            use_case::FilterTaskUseCaseComponent,
            schedule::ClientComponent,
            schedule::SchedulerComponent,
        ],
        providers = [],
    }
}

pub fn app_module(database_url: String) -> ModuleBuilder<AppModule> {
    AppModule::builder().with_component_parameters::<DatabaseUrl>(database_url)
}

pub type CreateTaskUseCase =
    domain::CreateTaskUseCase<Arc<dyn TaskRepository>, Arc<dyn IdRepository>>;

pub type ReadTaskUseCase = domain::ReadTaskUseCase<Arc<dyn TaskRepository>>;

pub type FilterTaskUseCase = domain::FilterTaskUseCase<Arc<dyn TaskRepository>>;

pub type UpdateTaskUseCase = domain::UpdateTaskUseCase<Arc<dyn TaskRepository>>;

pub type DeleteTaskUseCase = domain::DeleteTaskUseCase<Arc<dyn TaskRepository>>;

mod domain {
    pub use exam_task_domain::use_case::{
        CreateTaskUseCase, DeleteTaskUseCase, FilterTaskUseCase, ReadTaskUseCase, UpdateTaskUseCase,
    };
}
