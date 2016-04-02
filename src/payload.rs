//! DNS Payload struct and implementation

use header::Header;
use query::Query;
use rr::ResourceRecord;
use util::parse_name_into;

#[derive(Debug)]
pub struct Payload {
    pub questions: Vec<Query>,
    pub answer_rrs: Vec<ResourceRecord>,
    pub authority_rrs: Vec<ResourceRecord>,
    pub additional_rrs: Vec<ResourceRecord>,
}

impl Payload {
    pub fn new(hdr: &Header, data: &[u8]) -> Payload {
        let mut questions: Vec<Query> = Vec::new();
        let mut answer_rrs: Vec<ResourceRecord> = Vec::new();
        let mut authority_rrs: Vec<ResourceRecord> = Vec::new();
        let mut additional_rrs: Vec<ResourceRecord> = Vec::new();
        let mut i: u32 = 0;

        for _ in 0..hdr.total_questions {
            let mut name = String::new();
            i += parse_name_into(&data[(i as usize)..], &mut name);
            let q = Query::new(name, &data[(i as usize)..], &mut i);
            questions.push(q);
        }
        for _ in 0..hdr.total_answer_rrs {
            let rr = ResourceRecord::new(&data, &mut i);
            answer_rrs.push(rr);
        }
        for _ in 0..hdr.total_authority_rrs {
            let rr = ResourceRecord::new(&data[..], &mut i);
            authority_rrs.push(rr);
        }
        for _ in 0..hdr.total_additional_rrs {
            let rr = ResourceRecord::new(&data[..], &mut i);
            additional_rrs.push(rr);
        }
        Payload {
            questions: questions,
            answer_rrs: answer_rrs,
            authority_rrs: authority_rrs,
            additional_rrs: additional_rrs,
        }
    }
}
