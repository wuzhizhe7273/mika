use chrono::{DateTime, Utc, Offset, Local};
use uuid::Uuid;

use crate::{
    api::article,
    init::db,
    model::{
        req::{category::{CreateCategory, GetCategoryList, UpdateCategoryQuery}, PermQuery},
        resp::{category::CategoryVO, Pagination},
    },
    result::{AppResult, AppError},
};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, ActiveModelTrait,
};

pub struct CategoryDAO;
impl CategoryDAO {
    pub async fn create(query:PermQuery<CreateCategory>) -> AppResult<i64> {
        let c=query.req;
        let category = entity::category::ActiveModel {
            name: Set(c.name),
            ..Default::default()
        };
        let res = entity::category::Entity::insert(category)
            .exec(db())
            .await?;
        Ok(res.last_insert_id)
    }

    pub async fn detete(ids: Vec<Uuid>) -> AppResult<u64> {
        let res = entity::category::Entity::delete_many()
            .filter(entity::category::Column::Id.is_in(ids))
            .exec(db())
            .await?;
        Ok(res.rows_affected)
    }

    pub async fn list(r: GetCategoryList) -> AppResult<Pagination<CategoryVO>> {
        let order_by = match r.orderby.as_str() {
            "name" => entity::category::Column::Name,
            "updated_at" => entity::category::Column::UpdatedAt,
            "created_at" => entity::category::Column::CreatedAt,
            _ => entity::category::Column::UpdatedAt,
        };
        let order = match r.order {
            true => Order::Asc,
            false => Order::Desc,
        };
        let pages = entity::category::Entity::find()
            .column_as(entity::article::Column::Id.count(), "article_count")
            .join(
                JoinType::LeftJoin,
                entity::category::Relation::Article.def(),
            )
            .filter(entity::category::Column::Name.contains(r.name))
            .group_by(entity::category::Column::Id)
            .order_by(order_by, order)
            .into_model::<CategoryVO>()
            .paginate(db(), r.page_size);
        let total = pages.num_items().await?;
        let categories = pages.fetch_page(r.page - 1).await?;
        Ok(Pagination {
            total: total,
            page: r.page,
            data: categories,
        })
    }

    pub async fn delete(ids: Vec<Uuid>) -> AppResult<u64> {
        let rows = entity::category::Entity::delete_many()
            .filter(entity::category::Column::Id.is_in(ids))
            .exec(db())
            .await?
            .rows_affected;
        Ok(rows)
    }

    pub async fn update(req:UpdateCategoryQuery)->AppResult<i64>{
        let c=entity::category::Entity::find_by_id(req.id).one(db()).await?;
        let mut c:entity::category::ActiveModel=c.ok_or(AppError::NotFound)?.into();
        c.name=Set(req.name);
        c.updated_at=Set(Utc::now().into());
        let id=c.update(db()).await?.id;
        Ok(id)
    }
}
