use rubble::{
    att::{AttHandle, AttPermission, AttUuid, Attribute, AttributeProvider},
    uuid::{Uuid16, Uuid},
    Error,
    utils::HexSlice,
};

pub struct GattServer<'a> {
    attributes: [Attribute<'a>; 1],
}

impl<'a> GattServer<'a> {
    pub fn new() -> Self {
        Self {
            attributes: [Attribute {
                att_type: AttUuid::Uuid128(Uuid::from_bytes([0x00, 0x00,0x00,0xA2,0x00,0x00,0x10,0x00,0x80,0x00,0x00,0x26,0xBB,0x76,0x52,0x91])),
                handle: AttHandle::from_raw(0x0001),
                value: HexSlice(&[]),
                permission: AttPermission::default(),
            }],
        }
    }
}

impl<'a> AttributeProvider for GattServer<'a> {
    fn for_each_attr(
        &mut self,
        f: &mut dyn FnMut(&mut Attribute) -> Result<(), Error>,
    ) -> Result<(), Error> {
        for att in &mut self.attributes {
            f(att)?;
        }
        Ok(())
    }

    fn is_grouping_attr(&self, uuid: AttUuid) -> bool {
        uuid == Uuid16(0x2800)
    }

    fn group_end(&self, handle: AttHandle) -> Option<&Attribute> {
        for att in &self.attributes {
            if att.handle == handle && att.att_type == Uuid16(0x2800) {
                return Some(att);
            }
        }

        None
    }
}
