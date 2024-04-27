-- Add up migration script here

CREATE TABLE devices(
    device_library_id VARCHAR(255) PRIMARY KEY, 
    push_token VARCHAR(255) NOT NULL, 
    created_at TIMESTAMP NOT NULL,
    last_updated_at TIMESTAMP NOT NULL
);

CREATE TABLE passes(
    serial_number VARCHAR(255) PRIMARY KEY,
    pass_type_id VARCHAR(255) NOT NULL,
    auth_token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_updated_at TIMESTAMP NOT NULL
);

CREATE TABLE device_pass_registrations(
    device_library_id VARCHAR(255) REFERENCES devices(device_library_id), 
    pass_serial_number VARCHAR(255) REFERENCES passes(serial_number),
    created_at TIMESTAMP NOT NULL,
    CONSTRAINT device_pass_registration_pk PRIMARY KEY(device_library_id,pass_serial_number)
);
