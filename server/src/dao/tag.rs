use chrono::Utc;
use sea_orm::{Set, EntityTrait, Order, JoinType, QuerySelect, RelationTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, ActiveModelTrait};
use sea_query::Expr;
use uuid::Uuid;

use crate::{model::{req::tag::{CreateTag, GetTagList, UpdateTagQuery}, resp::{tag::TagVO, Pagination}}, result::{AppResult, AppError}, init::db};

pub struct TagDAO;
impl TagDAO {
    pub async fn create(req: CreateTag) -> AppResult<i64> {
        let tag = entity::tag::ActiveModel {
            name: Set(req.name),
            ..Default::default()
        };
        let id=entity::tag::Entity::insert(tag).exec(db()).await?.last_insert_id;
        Ok(id)
    }
    pub async fn list(r: GetTagList) -> AppResult<Pagination<TagVO>> {
        let order = match r.order {
            true => Order::Asc,
            false => Order::Desc,
        };
        let pages = entity::tag::Entity::find()
            .column_as(entity::article::Column::Id.count(), "article_count")
            .join(
                JoinType::LeftJoin,
                entity::tag::Relation::RArticleTag.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::r_article_tag::Relation::Article.def()
            )
            .filter(entity::tag::Column::Name.contains(r.name))
            .group_by(entity::tag::Column::Id)
            .order_by(Expr::cust(r.orderby), order)
            .into_model::<TagVO>()
            .paginate(db(), r.page_size);
        let total = pages.num_items().await?;
        let categories = pages.fetch_page(r.page - 1).await?;
        Ok(Pagination  {
            total: total,
            page: r.page,
            data: categories,
        })
    }
    pub async fn delete(ids:Vec<Uuid>)->AppResult<u64>{
        let rows=entity::tag::Entity::delete_many().filter(entity::tag::Column::Id.is_in(ids)).exec(db()).await?.rows_affected;
        Ok(rows)
    }
    pub async fn update(req:UpdateTagQuery)->AppResult<i64>{
        let t=entity::tag::Entity::find_by_id(req.id).one(db()).await?;
        let mut t:entity::tag::ActiveModel=t.ok_or(AppError::NotFound)?.into();
        t.name=Set(req.name);
        t.updated_at=Set(Utc::now().into());
        let id=t.update(db()).await?.id;
        Ok(id)
    }
}
