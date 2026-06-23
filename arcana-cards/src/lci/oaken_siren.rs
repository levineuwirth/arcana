//! Oaken Siren — `{1}{U}` 1/2 Artifact Creature — Siren Pirate.
//!
//! * Flying, vigilance.
//! * `{T}: Add {U}.` (Spend this mana only to cast an artifact spell or
//!   activate an ability of an artifact source — that spend-restriction
//!   rider is a fidelity GAP; the mana production itself is wired.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oaken Siren");
    let siren = reg.interner_mut().intern("Siren");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: "Spend this mana only to cast an artifact spell or
            // activate an ability of an artifact source" — no mana-use
            // restriction field on the produced mana; the {U} is added
            // unrestricted.
            text: "{T}: Add {U}.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_blue,
        }),
    )
}

fn add_blue(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
