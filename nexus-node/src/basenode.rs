use actix::WeakRecipient;
use nexus_ids::Aid;
use nexus_list::ARList;
use nexus_message::Dispatch;

pub struct BaseNode {
    aid: Aid,
    krnl: Option<WeakRecipient<Dispatch>>,
    neighbors: ARList<Aid>,
}

impl BaseNode {
    pub fn new(aid: Aid) -> Self {
        Self {
            aid,
            krnl: None,
            neighbors: Default::default(),
        }
    }

    pub fn aid(&self) -> &Aid {
        &self.aid
    }

    pub fn do_send(&self, msg: Dispatch) {
        self.krnl
            .as_ref()
            .and_then(|recipient| recipient.upgrade())
            .map(|recipeint| recipeint.do_send(msg))
            .unwrap_or(())
    }

    pub fn reg_kernel(&mut self, recipient: &WeakRecipient<Dispatch>) {
        self.krnl = Some(recipient.clone())
    }

    pub fn reg_neighbor(&mut self, aid: &Aid) {
        self.neighbors += aid;
    }

    pub fn unreg_neighbor(&mut self, aid: &Aid) {
        self.neighbors -= aid;
    }
}
