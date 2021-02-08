CREATE TABLE user_login(
id bigserial PRIMARY KEY ,
firstname varchar(50) NOT NULL,
lastname varchar(50) NOT NULL,
username varchar(100) NOT NULL,
email varchar(100) NOT NULL,
mobile varchar(15) NOT NULL,
facebookconnect varchar(200) NULL,
googleconnect varchar(200) NULL,
password varchar(500) NOT NULL,
ip_address varchar(50) NOT NULL,
isactive boolean NULL ,
sort_order bigint NULL,
created_at TIMESTAMP default NOW() NULL,
created_by varchar(20) NULL,
updated_at TIMESTAMP default NOW() NULL,
updated_by varchar(20) NULL
);


CREATE TABLE config_holidays (
id bigserial PRIMARY KEY, 
holiday_date varchar(15) NOT NULL, 
holiday_desc varchar(50) NOT NULL, 
createdat timestamp DEFAULT now(), 
updatedat timestamp DEFAULT now()
);

