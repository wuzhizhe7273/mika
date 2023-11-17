use sea_orm_migration::{prelude::*, sea_orm::prelude::Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        // root用户 email:123456@test.com password:123456
        let root_id = Uuid::new_v4();
        let insert=Query::insert()
            .into_table(User::Table)
            .columns([User::Id,User::Username,User::Email,User::Password])
            .values_panic([root_id.into(),"root".into(),"123456@test.com".into(),"$argon2id$v=19$m=19456,t=2,p=1$JvlukLHJ6wfu4u5QkfodlA$jQYaVY7FUbDADZ0u+Z3aJv0SpNlh/Gi0IPIBGf8kSKI".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Role::Table)
                    .col(ColumnDef::new(Role::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Role::Desc).string().not_null().default(""))
                    .col(ColumnDef::new(Role::ParentId).uuid())
                    .col(
                        ColumnDef::new(Role::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Role::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_parent")
                            .from(Role::Table, Role::ParentId)
                            .to(Role::Table, Role::Id),
                    )
                    .to_owned(),
            )
            .await?;
        let role_id=Uuid::new_v4();
        let insert=Query::insert().into_table(Role::Table)
            .columns([Role::Id,Role::Name,Role::Desc])
            .values_panic([role_id.into(),"SuperUser".into(),"超级用户".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Resource::Table)
                    .col(ColumnDef::new(Resource::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Resource::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Resource::Method).string().not_null())
                    .col(ColumnDef::new(Resource::Href).string().not_null())
                    .col(
                        ColumnDef::new(Resource::Desc)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Resource::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Resource::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Menu::Table)
                    .col(ColumnDef::new(Menu::Id).primary_key().uuid().not_null())
                    .col(ColumnDef::new(Menu::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Menu::MenuType).string().not_null())
                    .col(ColumnDef::new(Menu::Desc).string().not_null().default(""))
                    .col(ColumnDef::new(Menu::ParentId).uuid())
                    .col(ColumnDef::new(Menu::Order).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Menu::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Menu::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_menu_parent")
                            .from(Menu::Table, Menu::ParentId)
                            .to(Menu::Table, Menu::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RUserRole::Table)
                    .col(ColumnDef::new(RUserRole::UserId).uuid().not_null())
                    .col(ColumnDef::new(RUserRole::RoleId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_user_role_user")
                            .from(RUserRole::Table, RUserRole::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_user_role_role")
                            .from(RUserRole::Table, RUserRole::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_r_user_role")
                            .table(RUserRole::Table)
                            .col(RUserRole::RoleId)
                            .col(RUserRole::UserId),
                    )
                    .to_owned(),
            )
            .await?;
        let insert=Query::insert()
            .into_table(RUserRole::Table)
            .columns([RUserRole::UserId,RUserRole::RoleId])
            .values_panic([root_id.into(),role_id.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RRoleMenu::Table)
                    .col(ColumnDef::new(RRoleMenu::RoleId).uuid().not_null())
                    .col(ColumnDef::new(RRoleMenu::MenuId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_role_menu_role")
                            .from(RRoleMenu::Table, RRoleMenu::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_role_menu_menu")
                            .from(RRoleMenu::Table, RRoleMenu::MenuId)
                            .to(Menu::Table, Menu::Id),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_r_role_menu")
                            .table(RRoleMenu::Table)
                            .col(RRoleMenu::RoleId)
                            .col(RRoleMenu::MenuId),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RRoleRersource::Table)
                    .col(ColumnDef::new(RRoleRersource::RoleId).uuid().not_null())
                    .col(ColumnDef::new(RRoleRersource::ResourceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_role_resource_resource")
                            .from(RRoleRersource::Table, RRoleRersource::ResourceId)
                            .to(Resource::Table, Resource::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_role_resource_role")
                            .from(RRoleRersource::Table, RRoleRersource::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_r_role_resource")
                            .table(RRoleRersource::Table)
                            .col(RRoleRersource::RoleId)
                            .col(RRoleRersource::ResourceId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Category::Table)
                    .col(ColumnDef::new(Category::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Category::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Article::Table)
                    .col(ColumnDef::new(Article::Id).not_null().uuid().primary_key())
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::UserId).uuid().not_null())
                    .col(ColumnDef::new(Article::CategoryId).uuid())
                    .col(
                        ColumnDef::new(Article::Desc)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Article::Content).string().not_null())
                    .col(
                        ColumnDef::new(Article::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Article::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_user")
                            .from(Article::Table, Article::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_artilce_category")
                            .from(Article::Table, Article::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Tag::Table)
                    .col(ColumnDef::new(Tag::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(
                        ColumnDef::new(Tag::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tag::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RArticleTag::Table)
                    .col(ColumnDef::new(RArticleTag::ArticleId).uuid().not_null())
                    .col(ColumnDef::new(RArticleTag::TagId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_article_tag_article")
                            .from(RArticleTag::Table, RArticleTag::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_article_tag_tag")
                            .from(RArticleTag::Table, RArticleTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_r_article_tag")
                            .table(RArticleTag::Table)
                            .col(RArticleTag::ArticleId)
                            .col(RArticleTag::TagId),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Comment::Table)
                    .col(ColumnDef::new(Comment::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comment::UserId).uuid().not_null())
                    .col(ColumnDef::new(Comment::ParentId).uuid())
                    .col(ColumnDef::new(Comment::ReplyId).uuid())
                    .col(ColumnDef::new(Comment::ArticleId).uuid().not_null())
                    .col(ColumnDef::new(Comment::Content).string().not_null())
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_user")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_parent")
                            .from(Comment::Table, Comment::ParentId)
                            .to(Comment::Table, Comment::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_reply")
                            .from(Comment::Table, Comment::ReplyId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_article")
                            .from(Comment::Table, Comment::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Resource::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RUserRole::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RRoleMenu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RRoleRersource::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RArticleTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    Nickname,
    Avatar,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
    ParentId,
}

#[derive(DeriveIden)]
enum Resource {
    Table,
    Id,
    Name,
    Method,
    Href,
    Desc,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Menu {
    Table,
    Id,
    Name,
    MenuType,
    Desc,
    Order,
    CreatedAt,
    UpdatedAt,
    ParentId,
}

#[derive(DeriveIden)]
enum RUserRole {
    Table,
    UserId,
    RoleId,
}

#[derive(DeriveIden)]
enum RRoleMenu {
    Table,
    RoleId,
    MenuId,
}

#[derive(DeriveIden)]
enum RRoleRersource {
    Table,
    RoleId,
    ResourceId,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    UserId,
    CategoryId,
    Title,
    Desc,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RArticleTag {
    Table,
    ArticleId,
    TagId,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    UserId,
    ReplyId,
    ArticleId,
    ParentId,
    Content,
    CreatedAt,
    UpdatedAt,
}
