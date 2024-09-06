import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Appointment {
  'id' : bigint,
  'patient_id' : bigint,
  'multimedia_content' : [] | [MultiMediaContent],
  'date_time' : bigint,
  'doctor_id' : bigint,
  'reason' : string,
}
export type Error = { 'InvalidInput' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } };
export interface MedicalRecord {
  'id' : bigint,
  'patient_id' : bigint,
  'lab_results' : string,
  'treatment_history' : string,
}
export interface Message {
  'id' : bigint,
  'receiver_id' : bigint,
  'content' : string,
  'multimedia_content' : [] | [MultiMediaContent],
  'sender_id' : bigint,
}
export interface MultiMediaContent {
  'image_url' : [] | [string],
  'audio_url' : [] | [string],
  'video_url' : [] | [string],
}
export interface Patient {
  'id' : bigint,
  'name' : string,
  'contact_details' : string,
  'medical_history' : string,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Appointment } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Message } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : Patient } |
  { 'Err' : Error };
export type Result_4 = { 'Ok' : MedicalRecord } |
  { 'Err' : Error };
export interface _SERVICE {
  'delete_medical_record' : ActorMethod<[bigint], Result>,
  'delete_message' : ActorMethod<[bigint], Result>,
  'delete_patient' : ActorMethod<[bigint], Result>,
  'get_appointment' : ActorMethod<[bigint], Result_1>,
  'get_message' : ActorMethod<[bigint], Result_2>,
  'get_patient' : ActorMethod<[bigint], Result_3>,
  'list_appointments' : ActorMethod<[], Array<Appointment>>,
  'list_medical_records' : ActorMethod<[], Array<MedicalRecord>>,
  'list_messages' : ActorMethod<[], Array<Message>>,
  'list_patients' : ActorMethod<[], Array<Patient>>,
  'register_patient' : ActorMethod<[string, string, string], Result_3>,
  'schedule_appointment' : ActorMethod<
    [bigint, bigint, bigint, string, [] | [MultiMediaContent]],
    Result_1
  >,
  'send_message' : ActorMethod<
    [bigint, bigint, string, [] | [MultiMediaContent]],
    Result_2
  >,
  'send_reminder_to_patient' : ActorMethod<
    [bigint, string, [] | [MultiMediaContent]],
    Result_2
  >,
  'update_medical_record' : ActorMethod<
    [bigint, bigint, string, string],
    Result_4
  >,
  'update_message' : ActorMethod<
    [bigint, bigint, bigint, string, [] | [MultiMediaContent]],
    Result_2
  >,
  'update_patient' : ActorMethod<[bigint, string, string, string], Result_3>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
