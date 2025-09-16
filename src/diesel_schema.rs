diesel::table! {
    m_biodata (id) {
        id -> Bigint,
        created_by -> Bigint,
        created_on -> Datetime,
        deleted_by -> Nullable<Bigint>,
        deleted_on -> Nullable<Datetime>,
        #[max_length = 255]
        fullname -> Nullable<Varchar>,
        image -> Nullable<Blob>,
        #[max_length = 255]
        image_path -> Nullable<Varchar>,
        is_delete -> Bool,
        #[max_length = 15]
        mobile_phone -> Nullable<Varchar>,
        modified_by -> Nullable<Bigint>,
        modified_on -> Nullable<Datetime>,
    }
}

diesel::table! {
    m_user (id) {
        id -> Bigint,
        biodata_id -> Nullable<Bigint>,
        created_by -> Bigint,
        created_on -> Datetime,
        deleted_by -> Nullable<Bigint>,
        deleted_on -> Nullable<Datetime>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        is_delete -> Bool,
        is_locked -> Nullable<Bool>,
        last_login -> Nullable<Datetime>,
        login_attempt -> Nullable<Integer>,
        modified_by -> Nullable<Bigint>,
        modified_on -> Nullable<Datetime>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        role_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    m_role (id) {
        id -> Bigint,
        #[max_length = 20]
        code -> Nullable<Varchar>,
        created_by -> Bigint,
        created_on -> Datetime,
        deleted_by -> Nullable<Bigint>,
        deleted_on -> Nullable<Datetime>,
        is_delete -> Bool,
        modified_by -> Nullable<Bigint>,
        modified_on -> Nullable<Datetime>,
        #[max_length = 20]
        name -> Nullable<Varchar>,
        level -> Nullable<Tinyint>,
    }
}

diesel::table! {
    m_file (id) {
        id -> Bigint,
        created_by -> Bigint,
        created_on -> Datetime,
        deleted_by -> Nullable<Bigint>,
        deleted_on -> Nullable<Datetime>,
        #[max_length = 100]
        file_name -> Nullable<Varchar>,
        file -> Nullable<Blob>,
        #[max_length = 255]
        file_path -> Nullable<Varchar>,
        
        module_id -> Nullable<Bigint>,
        is_delete -> Bool,
        #[max_length = 50]
        file_type -> Nullable<Varchar>,
        modified_by -> Nullable<Bigint>,
        modified_on -> Nullable<Datetime>,
    }
}


diesel::joinable!(m_user -> m_biodata (biodata_id));
diesel::joinable!(m_user -> m_role (role_id));

diesel::allow_tables_to_appear_in_same_query!(m_biodata, m_role, m_user,);
