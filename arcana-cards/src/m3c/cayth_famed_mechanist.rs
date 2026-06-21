//! Cayth, Famed Mechanist — `{1}{U}{R}{W}` 3/3 Legendary Creature —
//! Dwarf Artificer.
//! "Fabricate 1 (When this creature enters, put a +1/+1 counter on it or
//!   create a 1/1 colorless Servo artifact creature token.)"
//! "Other nontoken creatures you control have fabricate 1."
//! "{2}, {T}: Choose one — • Populate. • Proliferate."
//!
//! Fabricate is not a usable `KeywordAbility`, and there is no
//! `Effect::Fabricate` / Populate primitive — the ETB Fabricate and the
//! Populate mode are GAP'd. The static granting Fabricate to other
//! creatures is also GAP'd. The activated ability is modeled as a single
//! activation that Proliferates (the expressible mode); the Populate
//! choice is GAP'd inside it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cayth, Famed Mechanist");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — "Other nontoken creatures you control have fabricate 1."
    // No Fabricate keyword / static-grant primitive available.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fabricate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Choose one — Populate. Proliferate.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: populate_or_proliferate,
            }),
    )
}

fn etb_fabricate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Fabricate 1" — no Effect::Fabricate primitive (the modal
    // counter-or-Servo-token ETB choice is not expressible).
    Vec::new()
}

fn populate_or_proliferate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Populate mode — no Effect::Populate primitive. Activated
    // abilities have no modal dispatch on this card class, so only the
    // Proliferate mode is emitted.
    vec![Effect::Proliferate]
}
