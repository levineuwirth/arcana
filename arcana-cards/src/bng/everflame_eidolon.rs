//! Everflame Eidolon — `{1}{R}` 1/1 Enchantment Creature — Spirit.
//!
//! Oracle:
//! * Bestow {2}{R} — an alternative Aura cast cost; `Bestow` is not a
//!   supported `KeywordAbility` variant and there is no alt-cost field, so
//!   it is GAP'd (`keywords: vec![]`).
//! * {R}: This creature gets +1/+0 until end of turn. If it's an Aura,
//!   enchanted creature gets +1/+0 until end of turn instead. — activated
//!   mana ability pumping itself +1/+0. (The "if it's an Aura, enchanted
//!   creature instead" redirection is a Bestow nuance not modeled; pumps
//!   self, the common case while it's a creature.)
//! * Enchanted creature gets +1/+1. — a static Aura buff (only relevant
//!   while bestowed); GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Everflame Eidolon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Enchanted creature gets +1/+1." (Bestow aura buff.)

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}: This creature gets +1/+0 until end of turn. If it's an Aura, \
                   enchanted creature gets +1/+0 until end of turn instead."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_self,
        }),
    )
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
