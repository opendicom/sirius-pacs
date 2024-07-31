-- Your SQL goes here
CREATE TABLE patients (
  patient_pk            BIGINT NOT NULL AUTO_INCREMENT,
  patient_id            VARCHAR(64) BINARY NULL,
  patient_id_issuer     VARCHAR(16) BINARY NULL,
  patient_fname         VARCHAR(250) BINARY NULL,
  patient_gname         VARCHAR(250) BINARY NULL,
  patient_gender         VARCHAR(250) BINARY NULL,
  PRIMARY KEY (patient_pk)
);


CREATE TABLE studies (
  study_pk              BIGINT NOT NULL AUTO_INCREMENT,
  study_iuid            VARCHAR(64) BINARY NULL,
  study_id              VARCHAR(16) BINARY NULL,
  study_datetime        DATETIME NULL,
  study_desc            VARCHAR(64) BINARY NULL,
  patient_fk			BIGINT,
  PRIMARY KEY (study_pk),
  CONSTRAINT patient_fk FOREIGN KEY (patient_fk) REFERENCES patients(patient_pk)
);


CREATE TABLE series (
    series_pk           BIGINT AUTO_INCREMENT NOT NULL,
    series_iuid         VARCHAR(64) BINARY NOT NULL,
    series_no           INT,
    series_mod          VARCHAR(16) BINARY,
    series_desc         VARCHAR(16) BINARY,
    series_datetime     DATETIME NULL,
    study_fk            BIGINT,
    PRIMARY KEY (series_pk),
    CONSTRAINT study_fk FOREIGN KEY (study_fk) REFERENCES studies(study_pk)
);