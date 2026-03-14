PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE AlbumRoots
                    (id INTEGER PRIMARY KEY,
                    label TEXT,
                    status INTEGER NOT NULL,
                    type INTEGER NOT NULL,
                    identifier TEXT,
                    specificPath TEXT,
                    caseSensitivity INTEGER,
                    UNIQUE(identifier, specificPath));
INSERT INTO AlbumRoots VALUES(1,'pictures',0,1,'volumeid:?uuid=c4b2bf2a-d877-46dd-8af0-9dc3bdf1e860&fileuuid=39d5a212-9fb8-4959-9bbe-8cf747a54174','/home/natalie/src/rust-digikam-orm/etc/pictures',2);
CREATE TABLE Albums
                    (id INTEGER PRIMARY KEY,
                    albumRoot INTEGER NOT NULL,
                    relativePath TEXT NOT NULL,
                    date DATE,
                    caption TEXT,
                    collection TEXT,
                    icon INTEGER,
                    modificationDate DATETIME,
                    UNIQUE(albumRoot, relativePath));
INSERT INTO Albums VALUES(1,1,'/','2026-03-15',NULL,NULL,NULL,'2026-03-15T11:05:36.954');
INSERT INTO Albums VALUES(2,1,'/Nature/Animals','2025-04-06','','Nature',NULL,'2026-03-15T10:43:26.074');
INSERT INTO Albums VALUES(3,1,'/Art','2025-10-17','','Uncategorized Album',NULL,'2026-03-15T10:42:25.359');
INSERT INTO Albums VALUES(4,1,'/Nature','2026-03-03','','Nature',NULL,'2026-03-15T10:43:06.682');
INSERT INTO Albums VALUES(5,1,'/Trains','2025-11-09','','Travel',NULL,'2026-03-15T10:46:53.700');
CREATE TABLE Images
                    (id INTEGER PRIMARY KEY,
                    album INTEGER,
                    name TEXT NOT NULL,
                    status INTEGER NOT NULL,
                    category INTEGER NOT NULL,
                    modificationDate DATETIME,
                    fileSize INTEGER,
                    uniqueHash TEXT,
                    manualOrder INTEGER,
                    UNIQUE (album, name));
INSERT INTO Images VALUES(1,5,'Train Yard.jpg',1,1,'2026-03-15T10:31:42.675',7894078,'b7713fa617264aac36920a6ed12c152a',NULL);
INSERT INTO Images VALUES(2,5,'Buffet Cart.jpg',1,1,'2026-03-15T10:31:42.607',5437007,'83f79fdea2c4dcb07bd21fbe0359420c',NULL);
INSERT INTO Images VALUES(3,5,'Instrument Cluster.jpg',1,1,'2026-03-15T10:31:42.887',4629294,'728ea74ec46eaad53ff42da887ee8c82',NULL);
INSERT INTO Images VALUES(4,3,'Cages.jpg',1,1,'2026-03-15T10:31:43.103',6691030,'6123c029f20465d237f5b127d88ce869',NULL);
INSERT INTO Images VALUES(5,3,'Screen.jpg',1,1,'2026-03-15T10:31:42.799',11831051,'e331459646ef2a166eb0e596f7b5b529',NULL);
INSERT INTO Images VALUES(6,3,'Graffiti.jpg',1,1,'2026-03-15T10:31:42.852',4841336,'d41c30212fd999376e437d82a5f011ee',NULL);
INSERT INTO Images VALUES(7,3,'Penrith Conservatorium of Music.jpg',1,1,'2026-03-15T10:31:43.271',23493887,'65fd24902aeddf71f5028f337f90f437',NULL);
INSERT INTO Images VALUES(8,4,'Flowers.tif',1,1,'2026-03-15T10:25:09.149',30419560,'d4a7ad53c740f1ae5131470094a99ddb',NULL);
INSERT INTO Images VALUES(9,2,'Birds.jpg',1,1,'2026-03-15T10:31:43.019',11558709,'0fc4e8e2104b04451cbc899801561211',NULL);
INSERT INTO Images VALUES(10,3,'Etheral Girl on Horse.jpg',1,1,'2026-03-15T10:31:42.735',4240039,'9b415c29b55e411253fc78c0aaf174cb',NULL);
INSERT INTO Images VALUES(11,2,'Turtle.jpg',1,1,'2026-03-15T10:24:04.694',7762249,'438af200f87c019b5cab91635a5c3266',NULL);
INSERT INTO Images VALUES(12,5,'Train Museam Volunteer.jpg',1,1,'2026-03-15T10:15:29.041',6977170,'46ec5c06babbcfd55e4213d9e197abe8',NULL);
CREATE TABLE ImageInformation
                    (imageid INTEGER PRIMARY KEY,
                    rating INTEGER,
                    creationDate DATETIME,
                    digitizationDate DATETIME,
                    orientation INTEGER,
                    width INTEGER,
                    height INTEGER,
                    format TEXT,
                    colorDepth INTEGER,
                    colorModel INTEGER);
INSERT INTO ImageInformation VALUES(1,4,'2025-11-09T06:33:10.000','2025-11-09T06:33:10.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(2,3,'2025-11-09T07:46:21.000','2025-11-09T07:46:21.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(3,4,'2025-11-09T08:00:09.000','2025-11-09T08:00:09.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(4,5,'2025-12-28T12:51:40.000','2025-12-28T12:51:40.000',1,5430,3457,'JPG',8,5);
INSERT INTO ImageInformation VALUES(5,4,'2025-12-28T13:05:03.000','2025-12-28T13:05:03.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(6,4,'2025-10-17T11:47:52.000','2025-10-17T11:47:52.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(7,3,'2026-03-03T19:34:44.000','2026-03-03T19:34:44.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(8,4,'2026-03-03T19:48:57.000','2026-03-03T19:48:57.000',1,5600,3728,'TIFF',8,1);
INSERT INTO ImageInformation VALUES(9,4,'2025-04-06T12:58:41.000','2025-04-06T12:58:41.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(10,5,'2026-03-07T11:54:44.000','2026-03-07T11:54:44.000',1,5496,3574,'JPG',8,5);
INSERT INTO ImageInformation VALUES(11,4,'2025-04-06T12:21:08.000','2025-04-06T12:21:08.000',1,5600,3728,'JPG',8,5);
INSERT INTO ImageInformation VALUES(12,4,'2025-11-09T06:46:23.000','2025-11-09T06:46:23.000',1,5600,3728,'JPG',8,5);
CREATE TABLE ImageMetadata
                    (imageid INTEGER PRIMARY KEY,
                    make TEXT,
                    model TEXT,
                    lens TEXT,
                    aperture REAL,
                    focalLength REAL,
                    focalLength35 REAL,
                    exposureTime REAL,
                    exposureProgram INTEGER,
                    exposureMode INTEGER,
                    sensitivity INTEGER,
                    flash INTEGER,
                    whiteBalance INTEGER,
                    whiteBalanceColorTemperature INTEGER,
                    meteringMode INTEGER,
                    subjectDistance REAL,
                    subjectDistanceCategory INTEGER);
INSERT INTO ImageMetadata VALUES(1,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',6.2999999999999998223,50.0,75.0,0.016666666666666666435,1,1,200,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(2,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',5.5999999999999996447,50.0,75.0,0.10000000000000000555,1,1,200,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(3,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',5.5999999999999996447,50.0,75.0,0.2000000000000000111,1,1,200,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(4,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',5.0,16.0,24.0,0.4000000000000000222,1,1,200,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(5,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',4.0,16.0,24.0,1.0,1,1,200,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(6,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',5.0,16.0,24.0,0.025000000000000001387,1,1,100,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(7,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',3.5,16.0,24.0,0.066666666666666665741,1,1,500,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(8,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',3.5,16.0,24.0,0.5,1,1,250,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(9,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',5.0,98.0,147.0,0.0002500000000000000052,1,1,2500,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(10,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 16-50mm f/3.5-6.3 VR',6.2999999999999998223,50.0,75.0,0.050000000000000002775,1,1,400,16,1,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(11,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',5.2999999999999998223,155.0,232.0,0.0020000000000000000416,0,0,320,16,0,NULL,5,NULL,0);
INSERT INTO ImageMetadata VALUES(12,'NIKON CORPORATION','NIKON Z 50','NIKKOR Z DX 50-250mm f/4.5-6.3 VR',4.5,50.0,75.0,0.040000000000000000832,1,1,640,16,1,NULL,5,NULL,0);
CREATE TABLE VideoMetadata
                    (imageid INTEGER PRIMARY KEY,
                    aspectRatio TEXT,
                    audioBitRate TEXT,
                    audioChannelType TEXT,
                    audioCompressor TEXT,
                    duration TEXT,
                    frameRate TEXT,
                    exposureProgram INTEGER,
                    videoCodec TEXT);
CREATE TABLE ImagePositions
                    (imageid INTEGER PRIMARY KEY,
                    latitude TEXT,
                    latitudeNumber REAL,
                    longitude TEXT,
                    longitudeNumber REAL,
                    altitude REAL,
                    orientation REAL,
                    tilt REAL,
                    roll REAL,
                    accuracy REAL,
                    description TEXT);
CREATE TABLE ImageComments
                    (id INTEGER PRIMARY KEY,
                    imageid INTEGER,
                    type INTEGER,
                    language TEXT,
                    author TEXT,
                    date DATETIME,
                    comment TEXT,
                    UNIQUE(imageid, type, language, author));
CREATE TABLE ImageCopyright
                    (id INTEGER PRIMARY KEY,
                    imageid INTEGER,
                    property TEXT,
                    value TEXT,
                    extraValue TEXT,
                    UNIQUE(imageid, property, value, extraValue));
CREATE TABLE Tags
                    (id INTEGER PRIMARY KEY,
                    pid INTEGER,
                    name TEXT NOT NULL,
                    icon INTEGER,
                    iconkde TEXT,
                    UNIQUE (name, pid));
INSERT INTO Tags VALUES(1,0,'_Digikam_Internal_Tags_',NULL,NULL);
INSERT INTO Tags VALUES(2,1,'Current Version',NULL,NULL);
INSERT INTO Tags VALUES(3,1,'Version Always Visible',NULL,NULL);
INSERT INTO Tags VALUES(4,0,'People',NULL,NULL);
INSERT INTO Tags VALUES(5,4,'Unknown',NULL,NULL);
INSERT INTO Tags VALUES(6,4,'Ignored',NULL,NULL);
INSERT INTO Tags VALUES(7,4,'Unconfirmed',NULL,NULL);
INSERT INTO Tags VALUES(8,1,'Need Resolving History',NULL,NULL);
INSERT INTO Tags VALUES(9,1,'Need Tagging History Graph',NULL,NULL);
INSERT INTO Tags VALUES(18,1,'Color Label None',NULL,NULL);
INSERT INTO Tags VALUES(19,1,'Color Label Red',NULL,NULL);
INSERT INTO Tags VALUES(20,1,'Color Label Orange',NULL,NULL);
INSERT INTO Tags VALUES(21,1,'Color Label Yellow',NULL,NULL);
INSERT INTO Tags VALUES(22,1,'Color Label Green',NULL,NULL);
INSERT INTO Tags VALUES(23,1,'Color Label Blue',NULL,NULL);
INSERT INTO Tags VALUES(24,1,'Color Label Magenta',NULL,NULL);
INSERT INTO Tags VALUES(25,1,'Color Label Gray',NULL,NULL);
INSERT INTO Tags VALUES(26,1,'Color Label Black',NULL,NULL);
INSERT INTO Tags VALUES(27,1,'Color Label White',NULL,NULL);
INSERT INTO Tags VALUES(28,1,'Pick Label None',NULL,NULL);
INSERT INTO Tags VALUES(29,1,'Pick Label Rejected',NULL,NULL);
INSERT INTO Tags VALUES(30,1,'Pick Label Pending',NULL,NULL);
INSERT INTO Tags VALUES(31,1,'Pick Label Accepted',NULL,NULL);
INSERT INTO Tags VALUES(44,1,'Scanned for Faces',NULL,NULL);
INSERT INTO Tags VALUES(45,0,'Animal',NULL,NULL);
INSERT INTO Tags VALUES(46,45,'Turtle',NULL,NULL);
INSERT INTO Tags VALUES(47,45,'Bird',NULL,NULL);
INSERT INTO Tags VALUES(48,0,'Art',NULL,NULL);
INSERT INTO Tags VALUES(49,0,'Size',NULL,NULL);
INSERT INTO Tags VALUES(50,49,'Wallpaper',NULL,NULL);
INSERT INTO Tags VALUES(51,0,'Flowers',NULL,NULL);
CREATE TABLE TagsTree
                    (id INTEGER NOT NULL,
                    pid INTEGER NOT NULL,
                    UNIQUE (id, pid));
INSERT INTO TagsTree VALUES(1,0);
INSERT INTO TagsTree VALUES(2,0);
INSERT INTO TagsTree VALUES(2,1);
INSERT INTO TagsTree VALUES(3,0);
INSERT INTO TagsTree VALUES(3,1);
INSERT INTO TagsTree VALUES(4,0);
INSERT INTO TagsTree VALUES(5,0);
INSERT INTO TagsTree VALUES(5,4);
INSERT INTO TagsTree VALUES(6,0);
INSERT INTO TagsTree VALUES(6,4);
INSERT INTO TagsTree VALUES(7,0);
INSERT INTO TagsTree VALUES(7,4);
INSERT INTO TagsTree VALUES(8,0);
INSERT INTO TagsTree VALUES(8,1);
INSERT INTO TagsTree VALUES(9,0);
INSERT INTO TagsTree VALUES(9,1);
INSERT INTO TagsTree VALUES(18,0);
INSERT INTO TagsTree VALUES(18,1);
INSERT INTO TagsTree VALUES(19,0);
INSERT INTO TagsTree VALUES(19,1);
INSERT INTO TagsTree VALUES(20,0);
INSERT INTO TagsTree VALUES(20,1);
INSERT INTO TagsTree VALUES(21,0);
INSERT INTO TagsTree VALUES(21,1);
INSERT INTO TagsTree VALUES(22,0);
INSERT INTO TagsTree VALUES(22,1);
INSERT INTO TagsTree VALUES(23,0);
INSERT INTO TagsTree VALUES(23,1);
INSERT INTO TagsTree VALUES(24,0);
INSERT INTO TagsTree VALUES(24,1);
INSERT INTO TagsTree VALUES(25,0);
INSERT INTO TagsTree VALUES(25,1);
INSERT INTO TagsTree VALUES(26,0);
INSERT INTO TagsTree VALUES(26,1);
INSERT INTO TagsTree VALUES(27,0);
INSERT INTO TagsTree VALUES(27,1);
INSERT INTO TagsTree VALUES(28,0);
INSERT INTO TagsTree VALUES(28,1);
INSERT INTO TagsTree VALUES(29,0);
INSERT INTO TagsTree VALUES(29,1);
INSERT INTO TagsTree VALUES(30,0);
INSERT INTO TagsTree VALUES(30,1);
INSERT INTO TagsTree VALUES(31,0);
INSERT INTO TagsTree VALUES(31,1);
INSERT INTO TagsTree VALUES(44,0);
INSERT INTO TagsTree VALUES(44,1);
INSERT INTO TagsTree VALUES(45,0);
INSERT INTO TagsTree VALUES(46,0);
INSERT INTO TagsTree VALUES(46,45);
INSERT INTO TagsTree VALUES(47,0);
INSERT INTO TagsTree VALUES(47,45);
INSERT INTO TagsTree VALUES(48,0);
INSERT INTO TagsTree VALUES(49,0);
INSERT INTO TagsTree VALUES(50,0);
INSERT INTO TagsTree VALUES(50,49);
INSERT INTO TagsTree VALUES(51,0);
CREATE TABLE ImageTags
                    (imageid INTEGER NOT NULL,
                    tagid INTEGER NOT NULL,
                    UNIQUE (imageid, tagid));
INSERT INTO ImageTags VALUES(1,31);
INSERT INTO ImageTags VALUES(1,18);
INSERT INTO ImageTags VALUES(2,31);
INSERT INTO ImageTags VALUES(2,18);
INSERT INTO ImageTags VALUES(3,31);
INSERT INTO ImageTags VALUES(3,18);
INSERT INTO ImageTags VALUES(8,31);
INSERT INTO ImageTags VALUES(9,31);
INSERT INTO ImageTags VALUES(9,18);
INSERT INTO ImageTags VALUES(11,30);
INSERT INTO ImageTags VALUES(11,24);
INSERT INTO ImageTags VALUES(9,47);
INSERT INTO ImageTags VALUES(11,45);
INSERT INTO ImageTags VALUES(11,46);
INSERT INTO ImageTags VALUES(9,45);
INSERT INTO ImageTags VALUES(5,15);
INSERT INTO ImageTags VALUES(5,32);
INSERT INTO ImageTags VALUES(5,34);
INSERT INTO ImageTags VALUES(5,48);
INSERT INTO ImageTags VALUES(7,15);
INSERT INTO ImageTags VALUES(7,48);
INSERT INTO ImageTags VALUES(5,31);
INSERT INTO ImageTags VALUES(5,18);
INSERT INTO ImageTags VALUES(7,31);
INSERT INTO ImageTags VALUES(7,18);
INSERT INTO ImageTags VALUES(1,11);
INSERT INTO ImageTags VALUES(1,12);
INSERT INTO ImageTags VALUES(1,15);
INSERT INTO ImageTags VALUES(1,16);
INSERT INTO ImageTags VALUES(1,17);
INSERT INTO ImageTags VALUES(1,49);
INSERT INTO ImageTags VALUES(1,50);
INSERT INTO ImageTags VALUES(2,11);
INSERT INTO ImageTags VALUES(2,12);
INSERT INTO ImageTags VALUES(2,15);
INSERT INTO ImageTags VALUES(2,16);
INSERT INTO ImageTags VALUES(2,17);
INSERT INTO ImageTags VALUES(2,49);
INSERT INTO ImageTags VALUES(2,50);
INSERT INTO ImageTags VALUES(3,11);
INSERT INTO ImageTags VALUES(3,12);
INSERT INTO ImageTags VALUES(3,15);
INSERT INTO ImageTags VALUES(3,16);
INSERT INTO ImageTags VALUES(3,17);
INSERT INTO ImageTags VALUES(3,49);
INSERT INTO ImageTags VALUES(3,50);
INSERT INTO ImageTags VALUES(8,50);
INSERT INTO ImageTags VALUES(8,51);
INSERT INTO ImageTags VALUES(12,18);
INSERT INTO ImageTags VALUES(6,31);
INSERT INTO ImageTags VALUES(6,18);
INSERT INTO ImageTags VALUES(6,15);
INSERT INTO ImageTags VALUES(6,48);
INSERT INTO ImageTags VALUES(6,50);
INSERT INTO ImageTags VALUES(4,31);
INSERT INTO ImageTags VALUES(4,18);
INSERT INTO ImageTags VALUES(4,11);
INSERT INTO ImageTags VALUES(4,15);
INSERT INTO ImageTags VALUES(4,32);
INSERT INTO ImageTags VALUES(4,33);
INSERT INTO ImageTags VALUES(4,34);
INSERT INTO ImageTags VALUES(4,48);
INSERT INTO ImageTags VALUES(4,50);
INSERT INTO ImageTags VALUES(10,31);
INSERT INTO ImageTags VALUES(10,18);
INSERT INTO ImageTags VALUES(10,15);
INSERT INTO ImageTags VALUES(10,48);
INSERT INTO ImageTags VALUES(10,50);
CREATE TABLE ImageProperties
                    (imageid  INTEGER NOT NULL,
                    property TEXT NOT NULL,
                    value    TEXT NOT NULL,
                    UNIQUE (imageid, property));
CREATE TABLE Searches
                    (id INTEGER PRIMARY KEY,
                    type INTEGER,
                    name TEXT NOT NULL,
                    query TEXT NOT NULL);
CREATE TABLE DownloadHistory
                    (id  INTEGER PRIMARY KEY,
                    identifier TEXT,
                    filename TEXT,
                    filesize INTEGER,
                    filedate DATETIME,
                    UNIQUE(identifier, filename, filesize, filedate));
INSERT INTO DownloadHistory VALUES(1,'d41d8cd98f00b204e9800998ecf8427e','2025-12-13-0011.jpg',7900515,'2026-03-15T10:29:53.777');
INSERT INTO DownloadHistory VALUES(2,'d41d8cd98f00b204e9800998ecf8427e','2025-12-13-0202.jpg',5443444,'2026-03-15T10:29:53.777');
INSERT INTO DownloadHistory VALUES(3,'d41d8cd98f00b204e9800998ecf8427e','2025-12-13-0248.jpg',4635731,'2026-03-15T10:29:53.777');
INSERT INTO DownloadHistory VALUES(4,'d41d8cd98f00b204e9800998ecf8427e','2025-12-28-0089.jpg',6699867,'2026-03-15T10:29:53.775');
INSERT INTO DownloadHistory VALUES(5,'d41d8cd98f00b204e9800998ecf8427e','2025-12-28-0110.jpg',11840148,'2026-03-15T10:29:53.776');
INSERT INTO DownloadHistory VALUES(6,'d41d8cd98f00b204e9800998ecf8427e','20251120_0006.jpg',4848333,'2026-03-15T10:29:53.775');
INSERT INTO DownloadHistory VALUES(7,'d41d8cd98f00b204e9800998ecf8427e','20260304_0011.jpg',23500189,'2026-03-15T10:29:53.776');
INSERT INTO DownloadHistory VALUES(8,'d41d8cd98f00b204e9800998ecf8427e','20260304_0040.tif',30419560,'2026-03-15T10:29:53.776');
INSERT INTO DownloadHistory VALUES(9,'d41d8cd98f00b204e9800998ecf8427e','DSC_0732.jpg',11565097,'2026-03-15T10:29:53.775');
INSERT INTO DownloadHistory VALUES(10,'d41d8cd98f00b204e9800998ecf8427e','Etheral Girl on Horse.jpg',4250411,'2026-03-15T10:29:53.776');
CREATE TABLE Settings
                    (keyword TEXT NOT NULL UNIQUE,
                    value TEXT);
INSERT INTO Settings VALUES('preAlpha010Update1','true');
INSERT INTO Settings VALUES('preAlpha010Update2','true');
INSERT INTO Settings VALUES('preAlpha010Update3','true');
INSERT INTO Settings VALUES('beta010Update1','true');
INSERT INTO Settings VALUES('beta010Update2','true');
INSERT INTO Settings VALUES('uniqueHashVersion','3');
INSERT INTO Settings VALUES('databaseImageFormats','jpg;jpeg;jpe;mpo;jp2;j2k;jpx;jpc;pgx;tif;tiff;png;exr;eps;fit;fts;fits;gif;xbm;xpm;ppm;pbm;pgm;pnm;pic;cur ;ico;icns;pgf;bmp;pcx;tga;sgi;rgb;rgba;bw;heic;heif;hif;jxl;avif;wbmp;webp;xcf;psd;psb;kra;ora;wmf;3fr;ari;arw;bay;bmq;cap;cin;cine;cr2;cr3;crw;cs1;dc2;dcr;dng;erf;fff;hdr;ia;iiq;k25;kc2;kdc;mdc;mef;mos;mrw;nef;nrw;orf;ori;pef;pxn;qtk;raf;raw;rdc;rw2;rwl;sr2;srf;srw;sti;x3f;');
INSERT INTO Settings VALUES('databaseVideoFormats','mpeg;mpg;mpe;mts;vob;avi;divx;wmv;asf;mp4;3gp;mov;3g2;m4v;m2v;mkv;webm;mng;m2ts;mxf');
INSERT INTO Settings VALUES('databaseAudioFormats','ogg;oga;flac;wv;ape;mpc;au;m4b;aax;aa;mp3;aac;m4a;m4p;caf;aiff;wma;wav');
INSERT INTO Settings VALUES('databaseIgnoreDirectoryFormats','@eaDir');
INSERT INTO Settings VALUES('FilterSettingsVersion','19');
INSERT INTO Settings VALUES('DcrawFilterSettingsVersion','8');
INSERT INTO Settings VALUES('databaseUUID','{b8d0d907-40ca-4f8e-97bf-05f65e55147c}');
INSERT INTO Settings VALUES('Locale','UTF-8');
INSERT INTO Settings VALUES('DBVersion','16');
INSERT INTO Settings VALUES('DBVersionRequired','16');
INSERT INTO Settings VALUES('DeleteRemovedCompleteScanCount','3');
INSERT INTO Settings VALUES('Scanned','2026-03-15T11:10:36');
CREATE TABLE ImageHistory
                    (imageid INTEGER PRIMARY KEY,
                    uuid TEXT,
                    history TEXT);
CREATE TABLE ImageRelations
                    (subject INTEGER,
                    object INTEGER,
                    type INTEGER,
                    UNIQUE(subject, object, type));
CREATE TABLE TagProperties
                    (tagid INTEGER,
                    property TEXT,
                    value TEXT);
INSERT INTO TagProperties VALUES(1,'internalTag',NULL);
INSERT INTO TagProperties VALUES(2,'internalTag',NULL);
INSERT INTO TagProperties VALUES(3,'internalTag',NULL);
INSERT INTO TagProperties VALUES(5,'person',NULL);
INSERT INTO TagProperties VALUES(5,'unknownPerson',NULL);
INSERT INTO TagProperties VALUES(6,'person',NULL);
INSERT INTO TagProperties VALUES(6,'ignoredPerson',NULL);
INSERT INTO TagProperties VALUES(7,'person',NULL);
INSERT INTO TagProperties VALUES(7,'unconfirmedPerson',NULL);
INSERT INTO TagProperties VALUES(8,'internalTag',NULL);
INSERT INTO TagProperties VALUES(9,'internalTag',NULL);
INSERT INTO TagProperties VALUES(18,'internalTag',NULL);
INSERT INTO TagProperties VALUES(19,'internalTag',NULL);
INSERT INTO TagProperties VALUES(20,'internalTag',NULL);
INSERT INTO TagProperties VALUES(21,'internalTag',NULL);
INSERT INTO TagProperties VALUES(22,'internalTag',NULL);
INSERT INTO TagProperties VALUES(23,'internalTag',NULL);
INSERT INTO TagProperties VALUES(24,'internalTag',NULL);
INSERT INTO TagProperties VALUES(25,'internalTag',NULL);
INSERT INTO TagProperties VALUES(26,'internalTag',NULL);
INSERT INTO TagProperties VALUES(27,'internalTag',NULL);
INSERT INTO TagProperties VALUES(28,'internalTag',NULL);
INSERT INTO TagProperties VALUES(29,'internalTag',NULL);
INSERT INTO TagProperties VALUES(30,'internalTag',NULL);
INSERT INTO TagProperties VALUES(31,'internalTag',NULL);
INSERT INTO TagProperties VALUES(44,'internalTag',NULL);
CREATE TABLE ImageTagProperties
                    (imageid INTEGER,
                    tagid INTEGER,
                    property TEXT,
                    value TEXT);
CREATE INDEX dir_index  ON Images (album);
CREATE INDEX hash_index ON Images (uniqueHash);
CREATE INDEX tag_index  ON ImageTags (tagid);
CREATE INDEX tag_id_index  ON ImageTags (imageid);
CREATE INDEX image_name_index ON Images (name);
CREATE INDEX creationdate_index ON ImageInformation (creationDate);
CREATE INDEX comments_imageid_index ON ImageComments (imageid);
CREATE INDEX copyright_imageid_index ON ImageCopyright (imageid);
CREATE INDEX uuid_index ON ImageHistory (uuid);
CREATE INDEX subject_relations_index ON ImageRelations (subject);
CREATE INDEX object_relations_index ON ImageRelations (object);
CREATE INDEX tagproperties_index ON TagProperties (tagid);
CREATE INDEX imagetagproperties_index ON ImageTagProperties (imageid, tagid);
CREATE INDEX imagetagproperties_imageid_index ON ImageTagProperties (imageid);
CREATE INDEX imagetagproperties_tagid_index ON ImageTagProperties (tagid);
CREATE TRIGGER delete_albumroot DELETE ON AlbumRoots
                    BEGIN
                        DELETE FROM Albums
                        WHERE Albums.albumRoot = OLD.id;
                    END;
CREATE TRIGGER delete_album DELETE ON Albums
                BEGIN
                    DELETE FROM Images
                    WHERE Images.album = OLD.id;
                END;
CREATE TRIGGER delete_image DELETE ON Images
                    BEGIN
                        DELETE FROM ImageTags          WHERE imageid=OLD.id;
                        DELETE From ImageInformation   WHERE imageid=OLD.id;
                        DELETE From ImageMetadata      WHERE imageid=OLD.id;
                        DELETE From VideoMetadata      WHERE imageid=OLD.id;
                        DELETE From ImagePositions     WHERE imageid=OLD.id;
                        DELETE From ImageComments      WHERE imageid=OLD.id;
                        DELETE From ImageCopyright     WHERE imageid=OLD.id;
                        DELETE From ImageProperties    WHERE imageid=OLD.id;
                        DELETE From ImageHistory       WHERE imageid=OLD.id;
                        DELETE FROM ImageRelations     WHERE subject=OLD.id OR object=OLD.id;
                        DELETE FROM ImageTagProperties WHERE imageid=OLD.id;
                        UPDATE Albums SET icon=null    WHERE icon=OLD.id;
                        UPDATE Tags SET icon=null      WHERE icon=OLD.id;
                    END;
CREATE TRIGGER delete_tag DELETE ON Tags
                    BEGIN
                        DELETE FROM ImageTags WHERE tagid=OLD.id;
                        DELETE FROM TagProperties WHERE tagid=OLD.id;
                        DELETE FROM ImageTagProperties WHERE tagid=OLD.id;
                    END;
CREATE TRIGGER insert_tagstree AFTER INSERT ON Tags
                    BEGIN
                    INSERT INTO TagsTree
                        SELECT NEW.id, NEW.pid
                        UNION
                        SELECT NEW.id, pid FROM TagsTree WHERE id=NEW.pid;
                    END;
CREATE TRIGGER delete_tagstree DELETE ON Tags
                    BEGIN
                        DELETE FROM Tags
                        WHERE id  IN (SELECT id FROM TagsTree WHERE pid=OLD.id);
                        DELETE FROM TagsTree
                        WHERE id IN (SELECT id FROM TagsTree WHERE pid=OLD.id);
                        DELETE FROM TagsTree
                        WHERE id=OLD.id;
                    END;
CREATE TRIGGER move_tagstree UPDATE OF pid ON Tags
                    BEGIN
                        DELETE FROM TagsTree
                            WHERE
                            ((id = OLD.id)
                            OR
                            id IN (SELECT id FROM TagsTree WHERE pid=OLD.id))
                            AND
                            pid IN (SELECT pid FROM TagsTree WHERE id=OLD.id);
                        INSERT INTO TagsTree
                            SELECT NEW.id, NEW.pid
                            UNION
                            SELECT NEW.id, pid FROM TagsTree WHERE id=NEW.pid
                            UNION
                            SELECT id, NEW.pid FROM TagsTree WHERE pid=NEW.id
                            UNION
                            SELECT A.id, B.pid FROM TagsTree A, TagsTree B
                            WHERE
                            A.pid = NEW.id AND B.id = NEW.pid;
                    END;
COMMIT;
