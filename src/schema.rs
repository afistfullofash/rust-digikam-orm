// THIS HAS BEEN HAND MODIFIED
// RERUNNING DIESIL WILL KILL CHANGES
#![allow(non_snake_case)]

diesel::table! {
    AlbumRoots (id) {
        id -> Nullable<Integer>,
        label -> Nullable<Text>,
        status -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        identifier -> Nullable<Text>,
    #[sql_name = "specificPath"]
        specific_path -> Nullable<Text>,
    #[sql_name = "caseSensitivity"]
        case_sensitivity -> Nullable<Integer>,
    }
}

diesel::table! {
    Albums (id) {
        id -> Nullable<Integer>,
    #[sql_name = "albumRoot"]
        album_root -> Integer,
    #[sql_name = "relativePath"]
        relative_path -> Text,
        date -> Nullable<Date>,
        caption -> Nullable<Text>,
        collection -> Nullable<Text>,
        icon -> Nullable<Integer>,
    #[sql_name = "modificationDate"]
        modification_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    DownloadHistory (id) {
        id -> Nullable<Integer>,
        identifier -> Nullable<Text>,
        filename -> Nullable<Text>,
        filesize -> Nullable<Integer>,
        filedate -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ImageComments (id) {
        id -> Nullable<Integer>,
        imageid -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        language -> Nullable<Text>,
        author -> Nullable<Text>,
        date -> Nullable<Timestamp>,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    ImageCopyright (id) {
        id -> Nullable<Integer>,
        imageid -> Nullable<Integer>,
        property -> Nullable<Text>,
        value -> Nullable<Text>,
        extraValue -> Nullable<Text>,
    }
}

diesel::table! {
    ImageHistory (imageid) {
        imageid -> Nullable<Integer>,
        uuid -> Nullable<Text>,
        history -> Nullable<Text>,
    }
}

diesel::table! {
    ImageInformation (imageid) {
        imageid -> Nullable<Integer>,
        rating -> Nullable<Integer>,
    #[sql_name = "creationDate"]
        creation_date -> Nullable<Timestamp>,
    #[sql_name = "digitizationDate"]
        digitizationDate -> Nullable<Timestamp>,
        orientation -> Nullable<Integer>,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        format -> Nullable<Text>,
    #[sql_name = "colorDepth"]
        color_depth -> Nullable<Integer>,
    #[sql_name = "colorModel"]
        color_model -> Nullable<Integer>,
    }
}

diesel::table! {
    ImageMetadata (imageid) {
        imageid -> Nullable<Integer>,
        make -> Nullable<Text>,
        model -> Nullable<Text>,
        lens -> Nullable<Text>,
        aperture -> Nullable<Float>,
    #[sql_name = "focalLength"]
        focal_length -> Nullable<Float>,
    #[sql_name = "focalLength35"]
        focal_length35 -> Nullable<Float>,
    #[sql_name = "exposureTime"]
        exposure_time -> Nullable<Float>,
    #[sql_name = "exposureProgram"]
        exposure_program -> Nullable<Integer>,
    #[sql_name = "exposureMode"]
        exposure_mode -> Nullable<Integer>,
        sensitivity -> Nullable<Integer>,
        flash -> Nullable<Integer>,
    #[sql_name = "whiteBalance"]
        white_balance -> Nullable<Integer>,
    #[sql_name = "whiteBalanceColorTemperature"]
        white_balance_color_temperature -> Nullable<Integer>,
    #[sql_name = "meteringMode"]
        metering_mode -> Nullable<Integer>,
    #[sql_name = "subjectDistance"]
        subject_distance -> Nullable<Float>,
    #[sql_name = "subjectDistanceCategory"]
        subject_distance_category -> Nullable<Integer>,
    }
}

diesel::table! {
    ImagePositions (imageid) {
        imageid -> Nullable<Integer>,
        latitude -> Nullable<Text>,
    #[sql_name = "latitudeNumber"]
        latitude_number -> Nullable<Float>,
        longitude -> Nullable<Text>,
    #[sql_name = "longitudeNumber"]
        longitude_number -> Nullable<Float>,
        altitude -> Nullable<Float>,
        orientation -> Nullable<Float>,
        tilt -> Nullable<Float>,
        roll -> Nullable<Float>,
        accuracy -> Nullable<Float>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    ImageProperties (rowid) {
        rowid -> Integer,
        imageid -> Integer,
        property -> Text,
        value -> Text,
    }
}

diesel::table! {
    ImageRelations (rowid) {
        rowid -> Integer,
        subject -> Nullable<Integer>,
        object -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
    }
}

diesel::table! {
    ImageTagProperties (rowid) {
        rowid -> Integer,
        imageid -> Nullable<Integer>,
        tagid -> Nullable<Integer>,
        property -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    ImageTags (rowid) {
        rowid -> Integer,
        imageid -> Integer,
        tagid -> Integer,
    }
}

diesel::table! {
    Images (id) {
        id -> Nullable<Integer>,
        album -> Nullable<Integer>,
        name -> Text,
        status -> Integer,
        category -> Integer,
    #[sql_name = "modificationDate"]
        modification_date -> Nullable<Timestamp>,
    #[sql_name = "fileSize"]
        file_size -> Nullable<Integer>,
    #[sql_name = "uniqueHash"]
        unique_hash -> Nullable<Text>,
    #[sql_name = "manualOrder"]
        manual_order -> Nullable<Integer>,
    }
}

diesel::table! {
    Searches (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        name -> Text,
        query -> Text,
    }
}

diesel::table! {
    Settings (rowid) {
        rowid -> Integer,
        keyword -> Text,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    TagProperties (rowid) {
        rowid -> Integer,
        tagid -> Nullable<Integer>,
        property -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    Tags (id) {
        id -> Nullable<Integer>,
        pid -> Nullable<Integer>,
        name -> Text,
        icon -> Nullable<Integer>,
        iconkde -> Nullable<Text>,
    }
}

diesel::table! {
    TagsTree (rowid) {
        rowid -> Integer,
        id -> Integer,
        pid -> Integer,
    }
}

diesel::table! {
    VideoMetadata (imageid) {
        imageid -> Nullable<Integer>,
    #[sql_name = "aspectRatio"]
        aspect_ratio -> Nullable<Text>,
    #[sql_name = "audioBitRate"]
        audio_bit_rate -> Nullable<Text>,
    #[sql_name = "audioChannelType"]
        audio_channel_type -> Nullable<Text>,
    #[sql_name = "audioCompressor"]
        audio_compressor -> Nullable<Text>,
        duration -> Nullable<Text>,
    #[sql_name = "frameRate"]
        frame_rate -> Nullable<Text>,
    #[sql_name = "exposureProgram"]
        exposure_program -> Nullable<Integer>,
    #[sql_name = "videoCodec"]
        video_codec -> Nullable<Text>,
    }
}

diesel::joinable!(ImageTags -> Images (imageid));
diesel::joinable!(ImageTags -> Tags (tagid));

diesel::allow_tables_to_appear_in_same_query!(
    AlbumRoots,
    Albums,
    DownloadHistory,
    ImageComments,
    ImageCopyright,
    ImageHistory,
    ImageInformation,
    ImageMetadata,
    ImagePositions,
    ImageProperties,
    ImageRelations,
    ImageTagProperties,
    ImageTags,
    Images,
    Searches,
    Settings,
    TagProperties,
    Tags,
    TagsTree,
    VideoMetadata,
);
