use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
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
        let insert=Query::insert()
            .into_table(User::Table)
            .columns([User::Username,User::Email,User::Password])
            .values_panic(["root".into(),"123456@test.com".into(),"$argon2id$v=19$m=19456,t=2,p=1$JvlukLHJ6wfu4u5QkfodlA$jQYaVY7FUbDADZ0u+Z3aJv0SpNlh/Gi0IPIBGf8kSKI".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Forum::Table)
                    .col(
                        ColumnDef::new(Forum::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Forum::ForumId).big_integer())
                    .col(ColumnDef::new(Forum::Name).string().not_null())
                    .col(ColumnDef::new(Forum::Desc).string().not_null().default(""))
                    .col(
                        ColumnDef::new(Forum::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Forum::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(ForeignKey::create().name("fk_forum_parent").from(Forum::Table, Forum::ForumId).to(Forum::Table, Forum::Id))
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(Forum::Table)
            .columns([Forum::Name])
            .values_panic(["root".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Role::Table)
                    .col(
                        ColumnDef::new(Role::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Role::DataPerm).tiny_integer().default(0))
                    .col(ColumnDef::new(Role::Rtype).tiny_integer().not_null().default(1))
                    .col(ColumnDef::new(Role::Desc).string().not_null().default(""))
                    .col(ColumnDef::new(Role::ParentId).big_integer())
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
        let insert = Query::insert()
            .into_table(Role::Table)
            .columns([Role::Name, Role::Rtype,Role::Desc,Role::DataPerm])
            .values_panic(["SuperUser".into(),0.into(), "超级用户".into(),0.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        manager.create_table(Table::create().if_not_exists().table(DataPerm::Table).col(ColumnDef::new(DataPerm::RoleId).big_integer().not_null()).col(ColumnDef::new(DataPerm::ForumId).big_integer().not_null()).foreign_key(ForeignKey::create().name("fk_dataperm_role").from(DataPerm::Table, DataPerm::RoleId).to(Role::Table, Role::Id)).foreign_key(ForeignKey::create().name("fk_dataperm_forum").from(DataPerm::Table, DataPerm::ForumId).to(Forum::Table, Forum::Id)).to_owned()).await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Resource::Table)
                    .col(
                        ColumnDef::new(Resource::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
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
                    .col(
                        ColumnDef::new(Menu::Id)
                            .primary_key()
                            .big_integer()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Menu::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(Menu::MenuType).string().not_null())
                    .col(ColumnDef::new(Menu::Desc).string().not_null().default(""))
                    .col(ColumnDef::new(Menu::ParentId).big_integer())
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
                    .table(RUserRoleForum::Table)
                    .col(
                        ColumnDef::new(RUserRoleForum::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RUserRoleForum::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RUserRoleForum::ForumId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_user_role_forum_user")
                            .from(RUserRoleForum::Table, RUserRoleForum::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_user_role_forum_role")
                            .from(RUserRoleForum::Table, RUserRoleForum::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_r_user_role_forum_forum")
                            .from(RUserRoleForum::Table, RUserRoleForum::ForumId)
                            .to(Forum::Table, Forum::Id),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_r_user_role")
                            .table(RUserRoleForum::Table)
                            .col(RUserRoleForum::RoleId)
                            .col(RUserRoleForum::UserId)
                            .col(RUserRoleForum::ForumId),
                    )
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(RUserRoleForum::Table)
            .columns([
                RUserRoleForum::UserId,
                RUserRoleForum::RoleId,
                RUserRoleForum::ForumId,
            ])
            .values_panic([1.into(), 1.into(), 1.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RRoleMenu::Table)
                    .col(ColumnDef::new(RRoleMenu::RoleId).big_integer().not_null())
                    .col(ColumnDef::new(RRoleMenu::MenuId).big_integer().not_null())
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
                    .col(
                        ColumnDef::new(RRoleRersource::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RRoleRersource::ResourceId)
                            .big_integer()
                            .not_null(),
                    )
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
                    .col(
                        ColumnDef::new(Category::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Category::ForumId).big_integer().not_null())
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_forum")
                            .from(Category::Table, Category::ForumId)
                            .to(Forum::Table, Forum::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Article::Table)
                    .col(
                        ColumnDef::new(Article::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Article::ForumId).big_integer().not_null())
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Article::CategoryId).big_integer())
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_forum")
                            .from(Article::Table, Article::ForumId)
                            .to(Forum::Table, Forum::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Tag::Table)
                    .col(
                        ColumnDef::new(Tag::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Tag::ForumId).big_integer().not_null())
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_forum")
                            .from(Tag::Table, Tag::ForumId)
                            .to(Forum::Table, Forum::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(RArticleTag::Table)
                    .col(
                        ColumnDef::new(RArticleTag::ArticleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RArticleTag::TagId).big_integer().not_null())
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
                    .col(
                        ColumnDef::new(Comment::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Comment::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Comment::ForumId).big_integer().not_null())
                    .col(ColumnDef::new(Comment::ParentId).big_integer())
                    .col(ColumnDef::new(Comment::ReplyId).big_integer())
                    .col(ColumnDef::new(Comment::ArticleId).big_integer().not_null())
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
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_forum")
                            .from(Comment::Table, Comment::ForumId)
                            .to(Forum::Table, Forum::Id),
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
            .drop_table(Table::drop().table(RUserRoleForum::Table).to_owned())
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
    Rtype,
    Desc,
    DataPerm, // 用户权限 0.本用户 1.本版块 2.本版块和子版块 3.全部板块 4.自定义板块
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
    ForumId,
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
    ForumId,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    ForumId,
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
    ForumId,
    ArticleId,
    ParentId,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Forum {
    Table,
    Id,
    ForumId,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RUserRoleForum {
    Table,
    UserId,
    RoleId,
    ForumId,
}

#[derive(DeriveIden)]
enum DataPerm{
    Table,
    RoleId,
    ForumId,
}