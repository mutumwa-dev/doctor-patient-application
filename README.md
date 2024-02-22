
# Doctor-Patient Application Documentation

## Overview
The Doctor-Patient Application is a web-based system designed to facilitate communication and management between doctors and patients within a healthcare environment. It provides features for doctors to schedule appointments, send messages, and manage patient records. Patients can register, view appointments, receive messages, and access their medical records through the application.

The application is built using Rust programming language with the Internet Computer (IC) Canister SDK, providing a secure and decentralized platform for healthcare data management. It utilizes stable data structures for efficient storage and retrieval of patient information, appointments, messages, and medical records.

## Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)
5. [Setting Up the Project](#setup)

## Dependencies <a name="dependencies"></a>
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Library for Candid serialization and deserialization.
- `ic_stable_structures`: Library providing stable data structures for the Internet Computer.
- `std`: Standard library for Rust.

## Data Structures <a name="data-structures"></a>
### Structs
1. `MultiMediaContent`: Represents multimedia content for communication, including image, video, and audio URLs.
2. `Patient`: Represents patient information, including ID, name, contact details, and medical history.
3. `Appointment`: Represents an appointment with details such as ID, patient ID, doctor ID, date/time, reason, and optional multimedia content.
4. `Message`: Represents a message with details like ID, sender ID, receiver ID, content, and optional multimedia content.
5. `MedicalRecord`: Represents a medical record with ID, patient ID, lab results, and treatment history.

### Enums
1. `Error`: Represents possible error types including "Not Found" and "Invalid Input".

## Functions <a name="functions"></a>
The Doctor-Patient Application provides various functions for managing doctor-patient interactions and healthcare data. Some key functions include:
- `get_patient`: Retrieve patient information by ID.
- `register_patient`: Register a new patient with provided details.
- `get_appointment`: Retrieve appointment information by ID.
- `schedule_appointment`: Schedule a new appointment.
- `get_message`: Retrieve message information by ID.
- `send_message`: Send a message to a recipient.
- `get_medical_record`: Retrieve medical record information by ID.
- `update_patient`: Update existing patient information.
- `delete_patient`: Delete a patient and associated records.
- `list_patients`: List all registered patients.
- `update_appointment`: Update existing appointment information.
- `delete_appointment`: Delete an appointment.
- `list_appointments`: List all scheduled appointments.
- Similar functions are available for messages and medical records management.

## Usage <a name="usage"></a>
The Doctor-Patient Application offers a user-friendly interface for doctors and patients to interact with the system. Key functionalities include:
- **Doctor Actions**:
  - Schedule appointments for patients.
  - Send messages to patients with reminders or instructions.
  - Update patient records with latest information.
- **Patient Actions**:
  - Register with the application to access services.
  - View scheduled appointments and receive reminders.
  - Communicate with doctors through messages.
  - Access and review personal medical records.

To use the application, users can navigate through the interface, perform desired actions, and interact with the system seamlessly. Proper error handling is implemented to handle cases such as invalid input or missing data.

## Setting Up the Project <a name="setup"></a>
To set up and start working on the Doctor-Patient Application project, follow these steps:

### 1. Install Rust and Dependencies
- Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
  ```bash
  $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  $ source "$HOME/.cargo/env"
  ```
- Install the `wasm32-unknown-unknown` target:
  ```bash
  $ rustup target add wasm32-unknown-unknown
  ```
- Install `candid-extractor`:
  ```bash
  $ cargo install candid-extractor
  ```

### 2. Install DFINITY SDK (`dfx`)
- Install `dfx` using the following commands:
  ```bash
  $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
  $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
  $ source ~/.bashrc
  $ dfx start --background
  ```

### 3. Update Dependencies
- Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

### 4. Autogenerate DID
- Add the provided script to the root directory of the project.
- Update line 16 with the name of your canister.
- Run the script each time you modify/add/remove exported functions of the canister.

### 5. Running the Project Locally
- Start the replica, running in the background:
  ```bash
  $ dfx start --background
  ```
- Deploy your canisters to the replica and generate your Candid interface:
  ```bash
  $ npm run gen-deploy
  ```