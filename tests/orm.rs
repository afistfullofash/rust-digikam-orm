use diesel::connection::SimpleConnection;
use diesel::{Connection, SqliteConnection};
use rust_digikam_orm::{Image, Tag};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn create_fixture_database() -> PathBuf {
    let sql = include_str!("fixtures/db.sql");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is before unix epoch")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("rust-digikam-orm-tests-{unique}.db"));
    let database_path = path.to_string_lossy().to_string();

    let mut connection =
        SqliteConnection::establish(&database_path).expect("failed to create sqlite database");
    connection
        .batch_execute(sql)
        .expect("failed to load fixture sql");

    path
}

#[test]
fn matches_tags_with_parent() {
    let fixture_path = create_fixture_database();
    let database_path = fixture_path.to_string_lossy().to_string();

    let tag = Tag::new(&database_path)
        .find_by_id(50)
        .expect("expected to find Wallpaper tag");
    let parent = tag.parent().expect("expected Wallpaper to have a parent");

    assert_eq!(tag.name(), "Wallpaper");
    assert_eq!(parent.id(), Some(49));
    assert_eq!(parent.name(), "Size");

    std::fs::remove_file(fixture_path).expect("failed to remove fixture database");
}

#[test]
fn resolves_full_tag_name_for_with_and_without_parent() {
    let fixture_path = create_fixture_database();
    let database_path = fixture_path.to_string_lossy().to_string();

    let wallpaper = Tag::new(&database_path)
        .find_by_id(50)
        .expect("expected to find Wallpaper tag");
    let art = Tag::new(&database_path)
        .find_by_id(48)
        .expect("expected to find Art tag");

    assert_eq!(wallpaper.full_name(), "/Size/Wallpaper");
    assert_eq!(art.full_name(), "/Art");

    std::fs::remove_file(fixture_path).expect("failed to remove fixture database");
}

#[test]
fn filters_by_multiple_tags_and_returns_expected_images() {
    let fixture_path = create_fixture_database();
    let database_path = fixture_path.to_string_lossy().to_string();

    let query_tags = vec!["/Art".to_string(), "/Size/Wallpaper".to_string()];
    let images = Image::new(&database_path).find_by_tag_strings(&query_tags);

    let mut image_names = images
        .into_iter()
        .map(|image| image.name())
        .collect::<Vec<_>>();
    image_names.sort();

    assert_eq!(
        image_names,
        vec![
            "Cages.jpg".to_string(),
            "Etheral Girl on Horse.jpg".to_string(),
            "Graffiti.jpg".to_string(),
        ]
    );

    std::fs::remove_file(fixture_path).expect("failed to remove fixture database");
}

#[test]
fn returns_image_by_id() {
    let fixture_path = create_fixture_database();
    let database_path = fixture_path.to_string_lossy().to_string();

    let image = Image::new(&database_path)
        .find_by_id(10)
        .expect("expected to find image with id 10");

    assert_eq!(image.id(), 10);
    assert_eq!(image.name(), "Etheral Girl on Horse.jpg");
    assert!(image.tags().iter().any(|tag| tag.full_name() == "/Art"));
    assert_eq!(
        image.path(),
        Some(
            "/home/natalie/src/rust-digikam-orm/etc/pictures/Art/Etheral Girl on Horse.jpg"
                .to_string()
        )
    );

    std::fs::remove_file(fixture_path).expect("failed to remove fixture database");
}
