use sea_orm::entity::prelude::*;

pub(crate) mod projects {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "projects")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub(crate) id: String,
        pub(crate) name: String,
        pub(crate) path: String,
        pub(crate) created_at: DateTimeUtc,
        pub(crate) active_version_id: Option<String>,
        pub(crate) active_config_id: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Documents,
        Versions,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Documents => Entity::has_many(super::documents::Entity).into(),
                Self::Versions => Entity::has_many(super::project_versions::Entity).into(),
            }
        }
    }

    impl Related<super::documents::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Documents.def()
        }
    }

    impl Related<super::project_versions::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Versions.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod project_versions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "project_versions")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub(crate) id: String,
        pub(crate) project_id: String,
        pub(crate) status: String,
        pub(crate) created_at: DateTimeUtc,
        pub(crate) item_count: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Project,
        Documents,
        Chunks,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Project => Entity::belongs_to(super::projects::Entity)
                    .from(Column::ProjectId)
                    .to(super::projects::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into(),
                Self::Documents => Entity::has_many(super::documents::Entity).into(),
                Self::Chunks => Entity::has_many(super::chunks::Entity).into(),
            }
        }
    }

    impl Related<super::projects::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Project.def()
        }
    }

    impl Related<super::documents::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Documents.def()
        }
    }

    impl Related<super::chunks::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Chunks.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod documents {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "documents")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub(crate) id: String,
        pub(crate) project_id: String,
        pub(crate) version_id: String,
        pub(crate) config_id: String,
        pub(crate) path: String,
        pub(crate) title: Option<String>,
        pub(crate) content_hash: String,
        pub(crate) indexed_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Project,
        Version,
        Chunks,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Project => Entity::belongs_to(super::projects::Entity)
                    .from(Column::ProjectId)
                    .to(super::projects::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into(),
                Self::Version => Entity::belongs_to(super::project_versions::Entity)
                    .from(Column::VersionId)
                    .to(super::project_versions::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into(),
                Self::Chunks => Entity::has_many(super::chunks::Entity).into(),
            }
        }
    }

    impl Related<super::projects::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Project.def()
        }
    }

    impl Related<super::project_versions::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Version.def()
        }
    }

    impl Related<super::chunks::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Chunks.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod chunks {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "chunks")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub(crate) id: String,
        pub(crate) doc_id: String,
        pub(crate) version_id: String,
        pub(crate) config_id: String,
        pub(crate) content: String,
        pub(crate) start_line: i32,
        pub(crate) end_line: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Document,
        Version,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Document => Entity::belongs_to(super::documents::Entity)
                    .from(Column::DocId)
                    .to(super::documents::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into(),
                Self::Version => Entity::belongs_to(super::project_versions::Entity)
                    .from(Column::VersionId)
                    .to(super::project_versions::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .into(),
            }
        }
    }

    impl Related<super::documents::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Document.def()
        }
    }

    impl Related<super::project_versions::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Version.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod indexing_configs {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "indexing_configs")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub(crate) config_id: String,
        pub(crate) chunking: String,
        pub(crate) embedding: String,
        pub(crate) vector_backend: Option<String>,
        pub(crate) vector_metric: String,
        pub(crate) index_params: Option<String>,
        pub(crate) created_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match *self {}
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
