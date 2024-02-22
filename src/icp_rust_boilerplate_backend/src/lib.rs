#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define MultimediaContent struct for multimedia communication
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct MultiMediaContent {
    image_url: Option<String>,
    video_url: Option<String>,
    audio_url: Option<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Patient {
    id: u64,
    name: String,
    contact_details: String,
    medical_history: String,
}

impl Storable for Patient {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Appointment {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    date_time: u64, 
    reason: String,
    multimedia_content: Option<MultiMediaContent>,
}

impl Storable for Appointment {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Appointment {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Message {
    id: u64,
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
}

impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct MedicalRecord {
    id: u64,
    patient_id: u64,
    lab_results: String,
    treatment_history: String,
}

impl Storable for MedicalRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MedicalRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PATIENT_STORAGE: RefCell<StableBTreeMap<u64, Patient, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static APPOINTMENT_STORAGE: RefCell<StableBTreeMap<u64, Appointment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static MESSAGE_STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static MEDICAL_RECORD_STORAGE: RefCell<StableBTreeMap<u64, MedicalRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

#[ic_cdk::query]
fn get_patient(patient_id: u64) -> Result<Patient, Error> {
    match _get_patient(&patient_id) {
        Some(patient) => Ok(patient),
        None => Err(Error::NotFound {
            msg: format!("patient with id={} not found", patient_id),
        }),
    }
}

#[ic_cdk::update]
fn register_patient(name: String, contact_details: String, medical_history: String) -> Result<Patient, Error> {
    // Validate input data
    if name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let patient = Patient { id, name, contact_details, medical_history };

    PATIENT_STORAGE.with(|service| service.borrow_mut().insert(id, patient.clone()));
    Ok(patient)
}

#[ic_cdk::query]
fn get_appointment(appointment_id: u64) -> Result<Appointment, Error> {
    match _get_appointment(&appointment_id) {
        Some(appointment) => Ok(appointment),
        None => Err(Error::NotFound {
            msg: format!("appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::update]
fn schedule_appointment(patient_id: u64, doctor_id: u64, date_time: u64, reason: String, multimedia_content: Option<MultiMediaContent>) -> Result<Appointment, Error> {
    // Validate input data
    if reason.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reason cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let appointment = Appointment {
        id,
        patient_id,
        doctor_id,
        date_time,
        reason,
        multimedia_content,
    };

    APPOINTMENT_STORAGE.with(|service| service.borrow_mut().insert(id, appointment.clone()));
    Ok(appointment)
}

#[ic_cdk::query]
fn get_message(message_id: u64) -> Result<Message, Error> {
    match _get_message(&message_id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::update]
fn send_message(sender_id: u64, receiver_id: u64, content: String, multimedia_content: Option<MultiMediaContent>) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let message = Message {
        id,
        sender_id,
        receiver_id,
        content,
        multimedia_content,
    };

    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));
    Ok(message)
}

#[ic_cdk::query]
fn get_medical_record(record_id: u64) -> Result<MedicalRecord, Error> {
    match _get_medical_record(&record_id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
fn update_patient(patient_id: u64, name: String, contact_details: String, medical_history: String) -> Result<Patient, Error> {
    // Validate input data
    if name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let updated_patient = Patient { id: patient_id, name, contact_details, medical_history };

    // Update patient in storage
    match PATIENT_STORAGE.with(|service| service.borrow_mut().insert(patient_id, updated_patient.clone())) {
        Some(_) => Ok(updated_patient),
        None => Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_patient(patient_id: u64) -> Result<(), Error> {
    // Remove patient from storage
    match PATIENT_STORAGE.with(|service| service.borrow_mut().remove(&patient_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        }),
    }
}

#[ic_cdk::query]
fn list_patients() -> Vec<Patient> {
    PATIENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, patient)| patient.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_appointment(appointment_id: u64, patient_id: u64, doctor_id: u64, date_time: u64, reason: String, multimedia_content: Option<MultiMediaContent>) -> Result<Appointment, Error> {
    // Validate input data
    if reason.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reason cannot be empty".to_string(),
        });
    }

    let updated_appointment = Appointment {
        id: appointment_id,
        patient_id,
        doctor_id,
        date_time,
        reason,
        multimedia_content,
    };

    // Update appointment in storage
    match APPOINTMENT_STORAGE.with(|service| service.borrow_mut().insert(appointment_id, updated_appointment.clone())) {
        Some(_) => Ok(updated_appointment),
        None => Err(Error::NotFound {
            msg: format!("Appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_appointment(appointment_id: u64) -> Result<(), Error> {
    // Remove appointment from storage
    match APPOINTMENT_STORAGE.with(|service| service.borrow_mut().remove(&appointment_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::query]
fn list_appointments() -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

// Similar implementation for messages and medical records

#[ic_cdk::update]
fn update_message(message_id: u64, sender_id: u64, receiver_id: u64, content: String, multimedia_content: Option<MultiMediaContent>) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let updated_message = Message {
        id: message_id,
        sender_id,
        receiver_id,
        content,
        multimedia_content,
    };

    // Update message in storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(message_id, updated_message.clone())) {
        Some(_) => Ok(updated_message),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_message(message_id: u64) -> Result<(), Error> {
    // Remove message from storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().remove(&message_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::query]
fn list_messages() -> Vec<Message> {
    MESSAGE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, message)| message.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_medical_record(record_id: u64, patient_id: u64, lab_results: String, treatment_history: String) -> Result<MedicalRecord, Error> {
    let updated_record = MedicalRecord {
        id: record_id,
        patient_id,
        lab_results,
        treatment_history,
    };

    // Update medical record in storage
    match MEDICAL_RECORD_STORAGE.with(|service| service.borrow_mut().insert(record_id, updated_record.clone())) {
        Some(_) => Ok(updated_record),
        None => Err(Error::NotFound {
            msg: format!("Medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_medical_record(record_id: u64) -> Result<(), Error> {
    // Remove medical record from storage
    match MEDICAL_RECORD_STORAGE.with(|service| service.borrow_mut().remove(&record_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::query]
fn list_medical_records() -> Vec<MedicalRecord> {
    MEDICAL_RECORD_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect()
    })
}

fn _get_patient(patient_id: &u64) -> Option<Patient> {
    PATIENT_STORAGE.with(|service| service.borrow().get(patient_id))
}

fn _get_appointment(appointment_id: &u64) -> Option<Appointment> {
    APPOINTMENT_STORAGE.with(|service| service.borrow().get(appointment_id))
}

fn _get_message(message_id: &u64) -> Option<Message> {
    MESSAGE_STORAGE.with(|service| service.borrow().get(message_id))
}

fn _get_medical_record(record_id: &u64) -> Option<MedicalRecord> {
    MEDICAL_RECORD_STORAGE.with(|service| service.borrow().get(record_id))
}

#[ic_cdk::update]
fn send_reminder_to_patient(patient_id: u64, content: String, multimedia_content: Option<MultiMediaContent>) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reminder content cannot be empty".to_string(),
        });
    }

    // Check if the patient exists
    if _get_patient(&patient_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        });
    }

    // Get the sender ID (could be a system ID or a doctor ID)
    let sender_id = 0; // You can change this based on your system design

    // Construct the message
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let message = Message {
        id,
        sender_id,
        receiver_id: patient_id,
        content,
        multimedia_content,
    };

    // Store the message
    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));

    Ok(message)
}

// Export Candid interface
ic_cdk::export_candid!();

