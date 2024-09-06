export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidInput' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const MultiMediaContent = IDL.Record({
    'image_url' : IDL.Opt(IDL.Text),
    'audio_url' : IDL.Opt(IDL.Text),
    'video_url' : IDL.Opt(IDL.Text),
  });
  const Appointment = IDL.Record({
    'id' : IDL.Nat64,
    'patient_id' : IDL.Nat64,
    'multimedia_content' : IDL.Opt(MultiMediaContent),
    'date_time' : IDL.Nat64,
    'doctor_id' : IDL.Nat64,
    'reason' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Appointment, 'Err' : Error });
  const Message = IDL.Record({
    'id' : IDL.Nat64,
    'receiver_id' : IDL.Nat64,
    'content' : IDL.Text,
    'multimedia_content' : IDL.Opt(MultiMediaContent),
    'sender_id' : IDL.Nat64,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Message, 'Err' : Error });
  const Patient = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'contact_details' : IDL.Text,
    'medical_history' : IDL.Text,
  });
  const Result_3 = IDL.Variant({ 'Ok' : Patient, 'Err' : Error });
  const MedicalRecord = IDL.Record({
    'id' : IDL.Nat64,
    'patient_id' : IDL.Nat64,
    'lab_results' : IDL.Text,
    'treatment_history' : IDL.Text,
  });
  const Result_4 = IDL.Variant({ 'Ok' : MedicalRecord, 'Err' : Error });
  return IDL.Service({
    'delete_medical_record' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_message' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_patient' : IDL.Func([IDL.Nat64], [Result], []),
    'get_appointment' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_message' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_patient' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'list_appointments' : IDL.Func([], [IDL.Vec(Appointment)], ['query']),
    'list_medical_records' : IDL.Func([], [IDL.Vec(MedicalRecord)], ['query']),
    'list_messages' : IDL.Func([], [IDL.Vec(Message)], ['query']),
    'list_patients' : IDL.Func([], [IDL.Vec(Patient)], ['query']),
    'register_patient' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [Result_3],
        [],
      ),
    'schedule_appointment' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Nat64, IDL.Text, IDL.Opt(MultiMediaContent)],
        [Result_1],
        [],
      ),
    'send_message' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Text, IDL.Opt(MultiMediaContent)],
        [Result_2],
        [],
      ),
    'send_reminder_to_patient' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Opt(MultiMediaContent)],
        [Result_2],
        [],
      ),
    'update_medical_record' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Text, IDL.Text],
        [Result_4],
        [],
      ),
    'update_message' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Nat64, IDL.Text, IDL.Opt(MultiMediaContent)],
        [Result_2],
        [],
      ),
    'update_patient' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text, IDL.Text],
        [Result_3],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
