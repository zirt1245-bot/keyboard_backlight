use windows::{
    core::{Result, BSTR, VARIANT},
    Win32::System::{
        Com::{
            CoCreateInstance, CoInitializeEx, CoInitializeSecurity, CoSetProxyBlanket,
            CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, EOAC_NONE, RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
        },
        Wmi::{IWbemClassObject, IWbemLocator, IWbemServices, WbemLocator, WBEM_GENERIC_FLAG_TYPE},
    },
};

const WMI_NAMESPACE: &str = r"root\WMI";
const WMI_INSTANCE: &str = r"CLEVO_GET.InstanceName='ACPI\PNP0C14\0_0'";
const METHOD_KB: &str = "SetKBLED";

#[allow(dead_code)]
const METHOD_SCF: &str = "SystemControlFunction";

pub struct WmiConnection {
    services: IWbemServices,
    class: IWbemClassObject,
    #[allow(dead_code)]
    instance: IWbemClassObject,
}

impl WmiConnection {
    pub fn new() -> Result<Self> {
        unsafe {
            // 1. Инициализация COM
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

            // 2. Инициализация безопасности COM — обязательно для WMI
            CoInitializeSecurity(
                None,                          // pSecDesc
                -1,                            // cAuthSvc (-1 = default)
                None,                          // asAuthSvc
                None,                          // pReserved1
                RPC_C_AUTHN_LEVEL_PKT_PRIVACY, // dwAuthnLevel
                RPC_C_IMP_LEVEL_IMPERSONATE,   // dwImpLevel
                None,                          // pAuthList
                EOAC_NONE,                     // dwCapabilities
                None,                          // pReserved3
            )
            .ok()
            .expect("Ошибка в WmiConnection");

            // 3. Создать WMI locator
            let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;

            // 4. Подключиться к root\WMI
            let services = locator.ConnectServer(
                &BSTR::from(WMI_NAMESPACE),
                None,
                None,
                None,
                0,
                None,
                None,
            )?;

            // 5. Установить прокси безопасность на services
            CoSetProxyBlanket(
                &services,
                10, // RPC_C_AUTHN_WINNT
                0,  // RPC_C_AUTHZ_NONE
                None,
                RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
            )?;

            // 6. Получить класс CLEVO_GET (для шаблона параметров методов)
            let mut class_obj: Option<IWbemClassObject> = None;
            services.GetObject(
                &BSTR::from("CLEVO_GET"),
                WBEM_GENERIC_FLAG_TYPE(0),
                None,
                Some(&mut class_obj),
                None,
            )?;
            let class = class_obj.expect("CLEVO_GET class not found");

            // 7. Получить конкретный инстанс
            let mut inst_obj: Option<IWbemClassObject> = None;
            services.GetObject(
                &BSTR::from(WMI_INSTANCE),
                WBEM_GENERIC_FLAG_TYPE(0),
                None,
                Some(&mut inst_obj),
                None,
            )?;
            let instance = inst_obj.expect("CLEVO_GET instance not found");

            Ok(Self {
                services,
                class,
                instance,
            })
        }
    }

    pub fn invoke_kb(&self, data: u32) -> Result<()> {
        self.invoke(METHOD_KB, data)
    }

    #[allow(dead_code)]
    pub fn invoke_scf(&self, data: u32) -> Result<()> {
        self.invoke(METHOD_SCF, data)
    }

    fn invoke(&self, method: &str, data: u32) -> Result<()> {
        unsafe {
            let method_bstr = BSTR::from(method);

            // Получить шаблон входных параметров из класса
            let mut in_params_def: Option<IWbemClassObject> = None;
            let mut out_params_def: Option<IWbemClassObject> = None;
            self.class
                .GetMethod(&method_bstr, 0, &mut in_params_def, &mut out_params_def)?;

            // Создать экземпляр параметров
            let in_params = in_params_def
                .as_ref()
                .expect("method has no in-params")
                .SpawnInstance(0)?;

            // Установить Data
            let val = VARIANT::from(data as i32);
            in_params.Put(&BSTR::from("Data"), 0, &val, 0)?;

            // Вызвать метод
            self.services.ExecMethod(
                &BSTR::from(WMI_INSTANCE),
                &method_bstr,
                WBEM_GENERIC_FLAG_TYPE(0),
                None,
                &in_params,
                None,
                None,
            )?;
        }
        Ok(())
    }
}
