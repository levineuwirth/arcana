//! Hashep Oasis — HOU "Land — Desert". "{T}: Add {C}." "{T}, Pay 1 life: Add
//! {G}." (The "{1}{G}{G}, {T}, Sacrifice a Desert: target creature gets +3/+3"
//! sorcery-speed pump is GAP'd — mana base only.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hashep Oasis");
    let desert = reg.interner_mut().intern("Desert");
    let mut subtypes = arcana_core::types::SubtypeSet::default();
    subtypes.0.insert(desert);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Pay 1 life: Add {G}.".into(),
                cost: ActivationCost { life: 1, ..ActivationCost::tap_only() },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            }),
    )
}

fn add_colorless(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)] }]
}
fn add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}
