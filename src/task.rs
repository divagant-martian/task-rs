use std::collections::HashMap;

//TODO make all of this static
// static default_project: String;
// default_due: String,
// default_scheduled: String,
// search_case_sensitive: bool,
// regex: bool,
// attributes: HashMap<String, String>, // name -> type
// coefficients: HashMap<String, f64>,
// custom_order: HashMap<String, Vec<String>>,
// urgency_project_coefficient: f64,
// urgency_active_coefficient: f64,
// urgency_scheduled_coefficient: f64,
// urgency_waiting_coefficient: f64,
// urgency_blocked_coefficient: f64,
// urgency_annotations_coefficient: f64,
// urgency_tags_coefficient: f64,
// urgency_due_coefficient: f64,
// urgency_blocking_coefficient: f64,
// urgency_age_coefficient: f64,

pub struct Task {
    data: HashMap<String, String>,
    id: usize,
    urgency_value: f64,
    recalc_urgency: bool,
    is_blocked: bool,
    is_blocking: bool,
    annotation_count: usize,
}

pub enum Status {
    Pending,
    Completed,
    Deleted,
    Recurring,
    Waiting,
}

pub enum DateState {
    DateNotDue,
    DateAfterToday,
    DateLaterToday,
    DateEarlierToday,
    DateBeforeToday,
}

impl Status {
    pub fn from_text(rep: &str) -> Self {
        todo!()
    }

    pub fn to_string(&self) -> String {
        todo!()
    }
}

impl Task {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_string(rep: &str) -> Self {
        todo!()
    }

    pub fn from_json(rep: &str) -> Self {
        // this is supposed to receive a "json::object*"
        todo!()
    }

    pub fn eq(&self, other: &Task) -> bool {
        // TODO this should be moved to the PartialEq impl block
        todo!()
    }

    pub fn parse(&self, inp: &str) {
        // no idea what this does / should do
        todo!()
    }

    pub fn compose_f4(&self) -> String {
        todo!()
    }

    pub fn compose_json(&self, decorate: bool) -> String {
        // TODO check if this can be done using serialization
        todo!()
    }
    pub fn set_as_now(&mut self, rep: &str) {
        todo!()
    }

    pub fn has(&self, pubsmt: &str) -> bool {
        todo!()
    }

    pub fn all(&self) -> Vec<String> {
        todo!()
    }

    pub fn identifier(&self, shortened: bool) -> String {
        todo!()
    }

    pub fn get(&self, smt: &str) -> String {
        todo!()
    }

      const std::string& get_ref (const std::string&) const;
  int get_int (const std::string&) const;
  unsigned long get_ulong (const std::string&) const;
  float get_float (const std::string&) const;
  time_t get_date (const std::string&) const;
  void set (const std::string&, const std::string&);
  void set (const std::string&, int);
  void remove (const std::string&);


}
