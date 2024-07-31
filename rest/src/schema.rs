// @generated automatically by Diesel CLI.

diesel::table! {
    patients (patient_pk) {
        patient_pk -> Bigint,
        #[max_length = 64]
        patient_id -> Nullable<Varchar>,
        #[max_length = 16]
        patient_id_issuer -> Nullable<Varchar>,
        #[max_length = 250]
        patient_fname -> Nullable<Varchar>,
        #[max_length = 250]
        patient_gname -> Nullable<Varchar>,
        #[max_length = 250]
        patient_gender -> Nullable<Varchar>,
    }
}

diesel::table! {
    series (series_pk) {
        series_pk -> Bigint,
        #[max_length = 64]
        series_iuid -> Varchar,
        series_no -> Nullable<Integer>,
        #[max_length = 16]
        series_mod -> Nullable<Varchar>,
        #[max_length = 16]
        series_desc -> Nullable<Varchar>,
        series_datetime -> Nullable<Datetime>,
        study_fk -> Nullable<Bigint>,
    }
}

diesel::table! {
    studies (study_pk) {
        study_pk -> Bigint,
        #[max_length = 64]
        study_iuid -> Nullable<Varchar>,
        #[max_length = 16]
        study_id -> Nullable<Varchar>,
        study_datetime -> Nullable<Datetime>,
        #[max_length = 64]
        study_desc -> Nullable<Varchar>,
        patient_fk -> Nullable<Bigint>,
    }
}

diesel::joinable!(series -> studies (study_fk));
diesel::joinable!(studies -> patients (patient_fk));

diesel::allow_tables_to_appear_in_same_query!(
    patients,
    series,
    studies,
);
