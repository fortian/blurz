use crate::bluetooth_device::BluetoothDevice;
use crate::bluetooth_le_advertising_data::BluetoothAdvertisingData;
use crate::bluetooth_session::BluetoothSession;
use crate::bluetooth_utils;
use crate::ok_or_str;
use dbus::arg::messageitem::MessageItem;
use dbus::Message;
use hex::FromHex;
use std::error::Error;

static ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";

#[derive(Clone, Debug)]
pub struct BluetoothAdapter<'a> {
    object_path: String,
    session: &'a BluetoothSession,
}

impl<'a> BluetoothAdapter<'a> {
    fn new(session: &'a BluetoothSession, object_path: &str) -> BluetoothAdapter<'a> {
        BluetoothAdapter {
            object_path: object_path.to_string(),
            session,
        }
    }

    pub fn init(session: &BluetoothSession) -> Result<BluetoothAdapter, Box<dyn Error>> {
        let adapters = bluetooth_utils::get_adapters(session.get_connection())?;

        if adapters.is_empty() {
            return Err(Box::from("Bluetooth adapter not found"));
        }

        Ok(BluetoothAdapter::new(session, &adapters[0]))
    }

    pub fn create_adapter(
        session: &'a BluetoothSession,
        object_path: &str,
    ) -> Result<BluetoothAdapter<'a>, Box<dyn Error>> {
        let adapters = bluetooth_utils::get_adapters(session.get_connection())?;

        for adapter in adapters {
            if adapter == object_path {
                return Ok(BluetoothAdapter::new(session, &adapter));
            }
        }
        Err(Box::from("Bluetooth adapter not found"))
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    pub fn get_first_device(&self) -> Result<BluetoothDevice, Box<dyn Error>> {
        let devices =
            bluetooth_utils::list_devices(self.session.get_connection(), &self.object_path)?;

        if devices.is_empty() {
            return Err(Box::from("No device found."));
        }
        Ok(BluetoothDevice::new(self.session, &devices[0]))
    }

    pub fn get_addata(&self) -> Result<BluetoothAdvertisingData, Box<dyn Error>> {
        let addata =
            bluetooth_utils::list_addata_1(self.session.get_connection(), &self.object_path)?;

        if addata.is_empty() {
            return Err(Box::from("No addata found."));
        }
        Ok(BluetoothAdvertisingData::new(&self.session, &addata[0]))
    }

    pub fn get_device_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        bluetooth_utils::list_devices(self.session.get_connection(), &self.object_path)
    }

    fn get_property(&self, prop: &str) -> Result<MessageItem, Box<dyn Error>> {
        bluetooth_utils::get_property(
            self.session.get_connection(),
            ADAPTER_INTERFACE,
            &self.object_path,
            prop,
        )
    }

    fn set_property<T>(&self, prop: &str, value: T, timeout_ms: i32) -> Result<(), Box<dyn Error>>
    where
        T: Into<MessageItem>,
    {
        bluetooth_utils::set_property(
            self.session.get_connection(),
            ADAPTER_INTERFACE,
            &self.object_path,
            prop,
            value,
            timeout_ms,
        )
    }

    fn call_method(
        &self,
        method: &str,
        param: Option<&[MessageItem]>,
        timeout_ms: i32,
    ) -> Result<Message, Box<dyn Error>> {
        bluetooth_utils::call_method(
            self.session.get_connection(),
            ADAPTER_INTERFACE,
            &self.object_path,
            method,
            param,
            timeout_ms,
        )
    }

    /*
     * Properties
     */

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n108
    pub fn get_address(&self) -> Result<String, Box<dyn Error>> {
        let address = self.get_property("Address")?;
        Ok(String::from(ok_or_str!(address.inner::<&str>())?))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n112
    pub fn get_name(&self) -> Result<String, Box<dyn Error>> {
        let name = self.get_property("Name")?;
        Ok(String::from(ok_or_str!(name.inner::<&str>())?))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n120
    pub fn get_alias(&self) -> Result<String, Box<dyn Error>> {
        let alias = self.get_property("Alias")?;
        Ok(String::from(ok_or_str!(alias.inner::<&str>())?))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n120
    pub fn set_alias(&self, value: &str) -> Result<(), Box<dyn Error>> {
        self.set_property("Alias", value, 1000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n139
    pub fn get_class(&self) -> Result<u32, Box<dyn Error>> {
        let class = self.get_property("Class")?;
        ok_or_str!(class.inner::<u32>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n147
    pub fn is_powered(&self) -> Result<bool, Box<dyn Error>> {
        let powered = self.get_property("Powered")?;
        ok_or_str!(powered.inner::<bool>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n147
    pub fn set_powered(&self, value: bool) -> Result<(), Box<dyn Error>> {
        self.set_property("Powered", value, 10000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n156
    pub fn is_discoverable(&self) -> Result<bool, Box<dyn Error>> {
        let discoverable = self.get_property("Discoverable")?;
        ok_or_str!(discoverable.inner::<bool>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n156
    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<dyn Error>> {
        self.set_property("Discoverable", value, 1000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n176
    pub fn is_pairable(&self) -> Result<bool, Box<dyn Error>> {
        let pairable = self.get_property("Pairable")?;
        ok_or_str!(pairable.inner::<bool>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n176
    pub fn set_pairable(&self, value: bool) -> Result<(), Box<dyn Error>> {
        self.set_property("Pairable", value, 1000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n187
    pub fn get_pairable_timeout(&self) -> Result<u32, Box<dyn Error>> {
        let pairable_timeout = self.get_property("PairableTimeout")?;
        ok_or_str!(pairable_timeout.inner::<u32>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n187
    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<dyn Error>> {
        self.set_property("PairableTimeout", value, 1000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n196
    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<dyn Error>> {
        let discoverable_timeout = self.get_property("DiscoverableTimeout")?;
        ok_or_str!(discoverable_timeout.inner::<u32>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n196
    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<dyn Error>> {
        self.set_property("DiscoverableTimeout", value, 1000)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n205
    pub fn is_discovering(&self) -> Result<bool, Box<dyn Error>> {
        let discovering = self.get_property("Discovering")?;
        ok_or_str!(discovering.inner::<bool>())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n209
    pub fn get_uuids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let uuids = self.get_property("UUIDs")?;
        let z: &[MessageItem] = ok_or_str!(uuids.inner())?;
        let mut v: Vec<String> = Vec::new();
        for y in z {
            v.push(String::from(ok_or_str!(y.inner::<&str>())?));
        }
        Ok(v)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n215
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<dyn Error>> {
        let modalias = self.get_property("Modalias")?;
        let m = ok_or_str!(modalias.inner::<&str>())?;
        let ids: Vec<&str> = m.split(':').collect();

        let source = String::from(ids[0]);
        let vendor = Vec::from_hex(ids[1][1..5].to_string())?;
        let product = Vec::from_hex(ids[1][6..10].to_string())?;
        let device = Vec::from_hex(ids[1][11..15].to_string())?;

        Ok((
            source,
            (vendor[0] as u32) * 16 * 16 + (vendor[1] as u32),
            (product[0] as u32) * 16 * 16 + (product[1] as u32),
            (device[0] as u32) * 16 * 16 + (device[1] as u32),
        ))
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<dyn Error>> {
        let (vendor_id_source, _, _, _) = self.get_modalias()?;
        Ok(vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<dyn Error>> {
        let (_, vendor_id, _, _) = self.get_modalias()?;
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32, Box<dyn Error>> {
        let (_, _, product_id, _) = self.get_modalias()?;
        Ok(product_id)
    }

    pub fn get_device_id(&self) -> Result<u32, Box<dyn Error>> {
        let (_, _, _, device_id) = self.get_modalias()?;
        Ok(device_id)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n12
    // Don't use this method, it's just a bomb now.
    //pub fn start_discovery(&self) -> Result<(), Box<dyn Error>> {
    //    Err(Box::from("Deprecated, use Discovery Session"))
    //}

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n27
    // Don't use this method, it's just a bomb now.
    //pub fn stop_discovery(&self) -> Result<(), Box<dyn Error>> {
    //    Err(Box::from("Deprecated, use Discovery Session"))
    //}

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/adapter-api.txt#n40
    pub fn remove_device(&self, device: &str) -> Result<(), Box<dyn Error>> {
        self.call_method(
            "RemoveDevice",
            Some(&[MessageItem::ObjectPath(device.to_string().into())]),
            1000,
        )?;
        Ok(())
    }

    // http://git.kernel.org/pub/scm/bluetooth/bluez.git/tree/doc/adapter-api.txt#n154
    pub fn connect_device(
        &self,
        address: &str,
        address_type: AddressType,
        timeout_ms: i32,
    ) -> Result<Message, Box<dyn Error>> {
        let address_type = match address_type {
            AddressType::Public => "public",
            AddressType::Random => "random",
        };

        let m = ok_or_str!(MessageItem::new_dict(vec![
            (
                "Address".into(),
                MessageItem::Variant(Box::new(address.into())),
            ),
            (
                "AddressType".into(),
                MessageItem::Variant(Box::new(address_type.into())),
            ),
        ]))?;

        self.call_method("ConnectDevice", Some(&[m]), timeout_ms)
    }
}

pub enum AddressType {
    Public,
    Random,
}
