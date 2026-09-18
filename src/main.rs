mod deserializer;
mod error;
mod lexer;
mod parser;

use deserializer::JsonDeserializer;

fn main() {
    let json = r#"{
    "CookieSPAEnabled": false,
    "CookieSameSiteNoneEnabled": false,
    "CookieV2CSPEnabled": false,
    "MultiVariantTestingEnabled": false,
    "UseV2": true,
    "MobileSDK": false,
    "SkipGeolocation": false,
    "ScriptType": "PRODUCTION",
    "Version": "202604.2.0",
    "OptanonDataJSON": "c3d9f1e3-55f3-4eba-b268-46cee4c6789c",
    "GeolocationUrl": "https://geolocation.onetrust.com/cookieconsentpub/v1/geo/location",
    "BulkDomainCheckUrl": "https://cookies-data.onetrust.io/bannersdk/v1/domaingroupcheck",
    "RuleSet": [
        {
            "Id": "019f1e88-e17e-70fb-a29d-2514ce0649ea",
            "Name": "EU/UK",
            "Countries": [
                "no",
                "de",
                "fi",
                "be",
                "pt",
                "bg",
                "dk",
                "lt",
                "lu",
                "lu",
                "lv",
                "hr",
                "fr",
                "hu",
                "se",
                "mc",
                "si",
                "sk",
                "mf",
                "sm",
                "gb",
                "yt",
                "ie",
                "gf",
                "ee",
                "mq",
                "ch",
                "mt",
                "gp",
                "is",
                "it",
                "gr",
                "es",
                "at",
                "re",
                "cy",
                "cz",
                "ax",
                "pl",
                "li",
                "ro",
                "nl"
            ],
            "States": {},
            "LanguageSwitcherPlaceholder": {
                "default": "en"
            },
            "BannerPushesDown": false,
            "Default": true,
            "Global": false,
            "Type": "IAB2V2",
            "UseGoogleVendors": false,
            "VariantEnabled": false,
            "TestEndTime": null,
            "Variants": [],
            "TemplateName": "Stack IAB TCF2.0 (TCF 2.2 Update) (2026 design)",
            "Conditions": [],
            "GCEnable": true,
            "IsGPPEnabled": false,
            "EnableJWTAuthForKnownUsers": false
        },
        {
            "Id": "019f1e88-de48-7993-9d44-02a6149fe258",
            "Name": "US National",
            "Countries": [
                "us"
            ],
            "States": {},
            "LanguageSwitcherPlaceholder": {
                "default": "en"
            },
            "BannerPushesDown": false,
            "Default": false,
            "Global": false,
            "Type": "USNATIONAL",
            "UseGoogleVendors": false,
            "VariantEnabled": false,
            "TestEndTime": null,
            "Variants": [],
            "TemplateName": "US National (2026 design) ",
            "Conditions": [],
            "GCEnable": true,
            "IsGPPEnabled": true,
            "EnableJWTAuthForKnownUsers": false
        }
    ]
}"#;

    match JsonDeserializer::new(json).deserialize() {
        Ok(parsed) => {
            println!("Successfully parsed JSON!\n");
            println!("{:#?}", parsed);
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {:?}", e);
        }
    }
}
